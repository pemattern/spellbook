use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::{ForkResult, execvp, fork, setsid},
};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Style},
    symbols,
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use std::{
    io::{self},
    process::exit,
    sync::mpsc,
    thread,
    time::Duration,
};

use crate::{
    config::{ColorMode, Config},
    message::Message,
    widgets::{
        application_list::{ApplicationList, ApplicationListState},
        counter::Counter,
        info::{Info, InfoState},
        input::{Input, InputState},
    },
};

#[derive(Debug)]
pub struct Spellbook {
    mode: RunMode,
    receiver: mpsc::Receiver<Message>,
    config: Config,
    state: SpellbookState,
}

#[derive(Debug, Default)]
enum RunMode {
    #[default]
    Running,
    Exit,
}

impl Spellbook {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        let mode = RunMode::Running;
        let config = Config::load();
        let state = SpellbookState::default();
        Self {
            mode,
            receiver,
            config,
            state,
        }
    }

    fn reload_config(&mut self) {
        self.config = Config::load();
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = ratatui::init();
        while let RunMode::Running = &self.mode {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_messages();
        }
        ratatui::restore();
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame
            .set_cursor_position(self.cursor_position(self.state.input.relative_cursor_position()));
        frame.render_widget(self, frame.area());
    }

    fn handle_messages(&mut self) {
        let message = self.receiver.recv().unwrap();
        match message {
            Message::Input(key_event) => self.handle_input(key_event),
            Message::Redraw => {}
            Message::ReloadConfig => self.reload_config(),
        }
    }

    fn handle_input(&mut self, key_event: KeyEvent) {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::ALT, KeyCode::Enter) => self.select_application(true),
            (KeyModifiers::ALT, KeyCode::Delete) => self.blacklist_application(),
            (_, KeyCode::Char(to_insert)) => {
                self.state.input.enter_char(to_insert);
                self.state.application_list.update(&self.state.input.filter);
            }
            (_, KeyCode::Backspace) => {
                self.state.input.delete_char();
                self.state.application_list.update(&self.state.input.filter);
            }
            (_, KeyCode::Delete) => {
                self.state.input.right_delete_char();
                self.state.application_list.update(&self.state.input.filter);
            }
            (_, KeyCode::Left) => self.state.input.move_cursor_left(),
            (_, KeyCode::Right) => self.state.input.move_cursor_right(),
            (_, KeyCode::Enter) => self.select_application(false),
            (_, KeyCode::Down | KeyCode::Tab) => self.move_selection_down(),
            (_, KeyCode::Up | KeyCode::BackTab) => self.move_selection_up(),
            (_, KeyCode::Esc) => self.mode = RunMode::Exit,
            _ => {}
        }
    }

    fn cursor_position(&self, relative_cursor_position: Position) -> Position {
        let icon_x = 3u16;
        let default_padding_x = 1u16;
        let input_border_x = 1u16;
        let x = self.config.margin.x
            + icon_x
            + default_padding_x
            + input_border_x
            + relative_cursor_position.x;

        let input_border_y = 1u16;
        let y = self.config.margin.y + input_border_y + relative_cursor_position.y;
        Position::new(x, y)
    }

    fn select_application(&mut self, keep_alive: bool) {
        let Some(application) = self.state.application_list.selected() else {
            return;
        };
        self.state
            .application_list
            .increment_launch_count(&application);
        self.state.application_list.save_db();
        if application.terminal {
            ratatui::restore();
            let _ = execvp(&application.filename, application.args.as_slice());
            return;
        }
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                if keep_alive {
                    return;
                }
                loop {
                    match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
                        Ok(WaitStatus::Exited(_, _)) => {
                            ratatui::restore();
                            exit(0);
                        }
                        Err(_) => todo!(),
                        _ => {
                            thread::sleep(Duration::from_millis(10));
                        }
                    }
                }
            }
            Ok(ForkResult::Child) => {
                let _ = setsid();
                match unsafe { fork() } {
                    Ok(ForkResult::Parent { child: _ }) => exit(0),
                    Ok(ForkResult::Child) => {
                        let _ = execvp(&application.filename, application.args.as_slice());
                    }
                    Err(_) => todo!(),
                }
            }
            Err(_) => todo!(),
        }
    }

    fn move_selection_down(&mut self) {
        self.state.application_list.select_next();
        self.set_info_to_current_application();
    }

    fn move_selection_up(&mut self) {
        self.state.application_list.select_previous();
        self.set_info_to_current_application();
    }

    fn set_info_to_current_application(&mut self) {
        let Some(application) = self.state.application_list.selected() else {
            return;
        };
        let message = application.comment;
        self.state.info.update_message(message);
    }

    fn blacklist_application(&mut self) {
        let application_list = &mut self.state.application_list;
        let Some(application) = application_list.selected() else {
            return;
        };
        application_list.blacklist(&application);
        application_list.save_db();
        application_list.update(&self.state.input.filter);
        let message = format!("blacklisted application '{}'", application.name);
        self.state.info.update_message(Some(message));
    }
}

impl Widget for &mut Spellbook {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if matches!(self.config.color_mode, ColorMode::Light) {
            buf.set_style(area, Style::new().bg(Color::Gray));
        }
        let margin = Margin::new(self.config.margin.x, self.config.margin.y);
        let padded_area = area.inner(margin);
        let main_block = Block::new();
        let [input_and_counter_area, list_area, info_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(if self.config.info.enable { 3 } else { 0 }),
        ])
        .areas(main_block.inner(padded_area));
        Widget::render(main_block, padded_area, buf);

        let fg_color = match self.config.color_mode {
            ColorMode::Light => Color::White,
            ColorMode::Dark => Color::Black,
        };
        let input_block = Block::new()
            .borders(Borders::all())
            .border_set(symbols::border::PROPORTIONAL_WIDE)
            .border_style(Style::new().fg(fg_color));
        let [input_area, counter_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Length(9)])
                .areas(input_block.inner(input_and_counter_area));
        Widget::render(input_block, input_and_counter_area, buf);

        StatefulWidget::render(
            Input::new(&self.config),
            input_area,
            buf,
            &mut self.state.input,
        );
        Widget::render(
            Counter::new(&self.config, &self.state.application_list),
            counter_area,
            buf,
        );
        StatefulWidget::render(
            ApplicationList::new(&self.config, &self.state.input),
            list_area,
            buf,
            &mut self.state.application_list,
        );
        let info_block = Block::new()
            .borders(Borders::all())
            .border_set(symbols::border::PROPORTIONAL_WIDE)
            .border_style(Style::new().fg(fg_color));
        Widget::render(info_block, info_area, buf);
        StatefulWidget::render(
            Info::new(&self.config),
            info_area.inner(Margin::new(1, 1)),
            buf,
            &mut self.state.info,
        );
    }
}

#[derive(Debug, Default)]
pub struct SpellbookState {
    pub input: InputState,
    pub application_list: ApplicationListState,
    pub info: InfoState,
}

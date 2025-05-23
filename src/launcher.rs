use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use nix::{
    sys::wait::{waitpid, WaitPidFlag, WaitStatus},
    unistd::{execvp, fork, setsid, ForkResult},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Position, Rect},
    widgets::{Block, Borders, StatefulWidget, Widget},
    Frame,
};
use std::{
    io::{self},
    process::exit,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::{
    config::Config,
    message::Message,
    widgets::{
        application_list::{ApplicationList, ApplicationListState},
        counter::Counter,
        debug::{Debug, DebugState},
        divider::Divider,
        input::{Input, InputState},
    },
};

#[derive(Debug)]
pub struct Launcher {
    mode: RunMode,
    receiver: mpsc::Receiver<Message>,
    config: Config,
    state: LauncherState,
}

#[derive(Debug, Default)]
enum RunMode {
    #[default]
    Running,
    Exit,
}

impl Launcher {
    pub fn new(receiver: mpsc::Receiver<Message>) -> Self {
        Self {
            mode: RunMode::Running,
            receiver,
            config: Config::load(),
            state: LauncherState::default(),
        }
    }

    fn reload_config(&mut self) {
        self.config = Config::load();
        self.state.debug.log("Reloaded config".to_string());
    }

    pub fn run(&mut self, start_time: Instant) -> io::Result<()> {
        let mut terminal = ratatui::init();
        self.state
            .debug
            .log(format!("Startup time: {:.2?}", start_time.elapsed()));
        while let RunMode::Running = &self.mode {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_messages()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame
            .set_cursor_position(self.cursor_position(self.state.input.relative_cursor_position()));
        frame.render_widget(self, frame.area());
    }

    fn handle_messages(&mut self) -> io::Result<()> {
        let message = self.receiver.recv().unwrap();
        match message {
            Message::Input(key_event) => self.handle_input(key_event)?,
            Message::Redraw => {}
            Message::ReloadConfig => self.reload_config(),
        }
        Ok(())
    }

    fn handle_input(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match (key_event.modifiers, key_event.code) {
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
            (KeyModifiers::ALT, KeyCode::Enter) => self.select_application(true),
            (_, KeyCode::Enter) => self.select_application(false),
            (_, KeyCode::Down | KeyCode::Tab) => self.state.application_list.select_next(),
            (_, KeyCode::Up | KeyCode::BackTab) => self.state.application_list.select_previous(),
            (_, KeyCode::Esc) => self.mode = RunMode::Exit,
            _ => {}
        }
        Ok(())
    }

    fn cursor_position(&self, relative_cursor_position: Position) -> Position {
        let border = &self.config.border;

        let margin_x = border.margin.x;
        let border_x = if border.enable_border { 1u16 } else { 0u16 };
        let icon_x = 3u16;
        let default_padding_x = 1u16;
        let x = margin_x + border_x + icon_x + default_padding_x + relative_cursor_position.x;

        let margin_y = border.margin.y;
        let border_y = if border.enable_border { 1u16 } else { 0u16 };
        let y = margin_y + border_y + relative_cursor_position.y;
        Position::new(x, y)
    }

    fn select_application(&mut self, keep_alive: bool) {
        let Some(application) = self.state.application_list.selected() else {
            return;
        };
        if application.terminal {
            ratatui::restore();
            let _ = execvp(&application.filename, application.args.as_slice());
            return;
        }
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => loop {
                match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
                    Ok(WaitStatus::Exited(_, _)) => {
                        if keep_alive {
                            return;
                        }
                        ratatui::restore();
                        exit(0);
                    }
                    Err(_) => todo!(),
                    _ => {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            },
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
}

impl Widget for &mut Launcher {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let margin = Margin::new(self.config.border.margin.x, self.config.border.margin.y);
        let padded_area = area.inner(margin);
        let mut main_block = Block::new();
        if self.config.border.enable_border {
            main_block = main_block.borders(Borders::ALL);
        }

        let [input_and_counter_area, divider_area, list_area, debug_divider_area, debug_area] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(if self.config.debug.enable { 1 } else { 0 }),
                Constraint::Length(if self.config.debug.enable { 1 } else { 0 }),
            ])
            .areas(main_block.inner(padded_area));
        Widget::render(main_block, padded_area, buf);
        let [input_area, _margin_area, counter_area] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(9),
        ])
        .areas(input_and_counter_area.inner(Margin::new(1, 0)));
        StatefulWidget::render(
            Input::new(&self.config.input),
            input_area,
            buf,
            &mut self.state.input,
        );
        Widget::render(Divider::new(&self.config.border), divider_area, buf);
        Widget::render(
            Counter::new(&self.config.counter, &self.state.application_list),
            counter_area,
            buf,
        );
        StatefulWidget::render(
            ApplicationList::new(&self.config.application_list, &self.state.input),
            list_area,
            buf,
            &mut self.state.application_list,
        );
        Widget::render(Divider::new(&self.config.border), debug_divider_area, buf);
        Widget::render(Debug::new(&self.state), debug_area, buf);
    }
}

#[derive(Debug, Default)]
pub struct LauncherState {
    pub input: InputState,
    pub application_list: ApplicationListState,
    pub debug: DebugState,
}

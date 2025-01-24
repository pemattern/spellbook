use std::{
    env,
    io::{self},
    os::unix::process::CommandExt,
    process::Command,
};

use fork::{fork, Fork};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Position, Rect},
    widgets::{Block, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};

use crate::{
    config::Config,
    widgets::{
        application_list::{ApplicationList, ApplicationListState},
        counter::{Counter, CounterState},
        divider::Divider,
        input::{Input, InputState},
    },
};

#[derive(Debug)]
pub struct Launcher {
    input_state: InputState,
    counter_state: CounterState,
    application_list_state: ApplicationListState,
    should_exit: bool,
}

impl Launcher {
    pub fn new(config: Config) -> Self {
        Self {
            input_state: InputState::from_config(&config),
            counter_state: CounterState::default(),
            application_list_state: ApplicationListState::default(),
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let index = self.input_state.cursor_index as u16;
        self.application_list_state
            .update_filter(&self.input_state.filter);
        self.counter_state = self.application_list_state.get_counter_state();
        frame.render_widget(self, frame.area());

        frame.set_cursor_position(Position::new(index + 2, 1));
    }

    fn select_entry(&mut self) {
        if let Some(application) = self.application_list_state.selected() {
            let shell = env::var("SHELL").expect("unable to read $SHELL env");
            if application.terminal {
                ratatui::restore();
                let _ = Command::new(&application.exec).exec();
            } else {
                let output = Command::new(&shell)
                    .args(&[
                        "-c",
                        format!("ps -o ppid= -p {}", std::process::id()).as_str(),
                    ])
                    .output()
                    .expect("unable to get ppid");
                match fork() {
                    Ok(Fork::Child) => {
                        let ppid = String::from_utf8_lossy(&output.stdout);
                        let _ = Command::new(&shell)
                            .args(&["-c", "sleep .1"])
                            .output()
                            .expect("...");
                        ratatui::restore();
                        let _ = Command::new(&shell)
                            .args(&["-c", format!("kill -9 {}", ppid).as_str()])
                            .status()
                            .expect("unable to kill terminal process");
                    }
                    Ok(Fork::Parent(_)) => {
                        let _ = Command::new(&shell)
                            .args(&["-c", format!("{} & disown", &application.exec).as_str()])
                            .exec();
                    }
                    Err(_) => panic!("fork failed"),
                }
            }
        }
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => self.input_state.enter_char(to_insert),
                    KeyCode::Backspace => self.input_state.delete_char(),
                    KeyCode::Delete => self.input_state.right_delete_char(),
                    KeyCode::Left => self.input_state.move_cursor_left(),
                    KeyCode::Right => self.input_state.move_cursor_right(),
                    KeyCode::Enter => self.select_entry(),
                    KeyCode::Down | KeyCode::Tab => self.application_list_state.select_next(),
                    KeyCode::Up | KeyCode::BackTab => self.application_list_state.select_previous(),
                    KeyCode::Esc => self.should_exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

impl Widget for &mut Launcher {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered();
        Widget::render(main_block, area, buf);

        let [filter_and_counter_area, divider_area, list_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(area.inner(Margin::new(1, 1)));
        let [filter_area, _, counter_area] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(self.counter_state.width()),
        ])
        .areas(filter_and_counter_area.inner(Margin::new(1, 0)));
        StatefulWidget::render(Input, filter_area, buf, &mut self.input_state);

        let divider = Divider::new('─');
        Widget::render(divider, divider_area, buf);

        StatefulWidget::render(Counter, counter_area, buf, &mut self.counter_state);

        StatefulWidget::render(
            ApplicationList,
            list_area,
            buf,
            &mut self.application_list_state,
        );
    }
}

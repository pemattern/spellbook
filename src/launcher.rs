use crossterm::event::KeyCode;
use nix::{
    sys::{
        signal::{kill, Signal},
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::{execvp, fork, getppid, ForkResult, Pid},
};
use procfs::process::Process;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    widgets::{Block, StatefulWidget, Widget},
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
            state: LauncherState::default(),
        }
    }

    pub fn run(&mut self, start_time: Instant) -> io::Result<()> {
        let mut terminal = ratatui::init();
        self.state
            .debug
            .log(format!("Startup time: {:.2?}", start_time.elapsed()));
        loop {
            match &self.mode {
                RunMode::Running => {
                    terminal.draw(|frame| self.draw(frame))?;
                    self.handle_messages()?;
                }
                RunMode::Exit => break,
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        frame.set_cursor_position(self.state.input.get_cursor_position());
        frame.render_widget(self, frame.area());
    }

    fn handle_messages(&mut self) -> io::Result<()> {
        let message = self.receiver.recv().unwrap();
        match message {
            Message::Input(key_code) => self.handle_input(key_code)?,
            Message::Redraw => {}
            Message::ReloadConfig => self.state.reload_config(),
        }
        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode) -> io::Result<()> {
        match code {
            KeyCode::Char(to_insert) => {
                self.state.input.enter_char(to_insert);
                self.state.application_list.update(&self.state.input.filter);
            }
            KeyCode::Backspace => {
                self.state.input.delete_char();
                self.state.application_list.update(&self.state.input.filter);
            }
            KeyCode::Delete => {
                self.state.input.right_delete_char();
                self.state.application_list.update(&self.state.input.filter);
            }
            KeyCode::Left => self.state.input.move_cursor_left(),
            KeyCode::Right => self.state.input.move_cursor_right(),
            KeyCode::Enter => self.select_application(),
            KeyCode::Down | KeyCode::Tab => self.state.application_list.select_next(),
            KeyCode::Up | KeyCode::BackTab => self.state.application_list.select_previous(),
            KeyCode::Esc => self.mode = RunMode::Exit,
            _ => {}
        }
        Ok(())
    }

    fn select_application(&mut self) {
        let Some(application) = self.state.application_list.selected() else {
            return;
        };
        if application.terminal {
            ratatui::restore();
            let _ = execvp(&application.filename, application.args.as_slice());
            return;
        }
        let shell_pid = getppid();
        let terminal_pid = Process::new(shell_pid.as_raw())
            .unwrap()
            .stat()
            .unwrap()
            .ppid;
        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => loop {
                match waitpid(child, Some(WaitPidFlag::WNOHANG)) {
                    Ok(WaitStatus::StillAlive) => {
                        let _ = kill(Pid::from_raw(terminal_pid), Signal::SIGTERM);
                        exit(0)
                    }
                    Err(_) => todo!(),
                    _ => {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            },
            Ok(ForkResult::Child) => {
                let _ = execvp(&application.filename, application.args.as_slice());
            }
            Err(_) => todo!(),
        }
    }
}

impl Widget for &mut Launcher {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered();
        Widget::render(main_block, area, buf);

        let [input_and_counter_area, divider_area, list_area, debug_divider_area, debug_area] =
            Layout::vertical([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .areas(area.inner(Margin::new(1, 1)));
        let [input_area, _margin_area, counter_area] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(9),
        ])
        .areas(input_and_counter_area.inner(Margin::new(1, 0)));
        StatefulWidget::render(Input, input_area, buf, &mut self.state);
        Widget::render(Divider::new(&self.state), divider_area, buf);
        Widget::render(Counter::new(&self.state), counter_area, buf);
        StatefulWidget::render(ApplicationList, list_area, buf, &mut self.state);
        Widget::render(Divider::new(&self.state), debug_divider_area, buf);
        Widget::render(Debug::new(&self.state), debug_area, buf);
    }
}

#[derive(Debug)]
pub struct LauncherState {
    pub config: Config,
    pub input: InputState,
    pub application_list: ApplicationListState,
    pub debug: DebugState,
}

impl LauncherState {
    fn reload_config(&mut self) {
        self.config = Config::load();
    }
}

impl Default for LauncherState {
    fn default() -> Self {
        Self {
            config: Config::load(),
            input: InputState::default(),
            application_list: ApplicationListState::default(),
            debug: DebugState::default(),
        }
    }
}

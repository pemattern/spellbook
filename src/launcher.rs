use crossterm::event::{self, Event, KeyCode, KeyEventKind};
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
    layout::{Constraint, Layout, Margin, Position, Rect},
    widgets::{Block, StatefulWidget, Widget},
    DefaultTerminal, Frame,
};
use std::{
    io::{self},
    mem,
    process::exit,
    thread,
    time::Duration,
};

use crate::{
    config::Config,
    watcher::Watcher,
    widgets::{
        application_list::{ApplicationList, ApplicationListState},
        counter::{Counter, CounterState},
        divider::Divider,
        input::{Input, InputState},
    },
};

#[derive(Debug)]
pub struct Launcher {
    state: LauncherState,
    input_state: InputState,
    counter_state: CounterState,
    application_list_state: ApplicationListState,
}

#[derive(Debug, Default)]
enum LauncherState {
    #[default]
    Running,
    ReloadConfig,
    Exit,
}

impl Launcher {
    pub fn new(config: &Config) -> Self {
        Watcher::watch();
        Self {
            state: LauncherState::Running,
            input_state: InputState::from_config(config),
            counter_state: CounterState::default(),
            application_list_state: ApplicationListState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            match &self.state {
                LauncherState::Running => {
                    terminal.draw(|frame| self.draw(frame))?;
                    self.handle_input()?;
                }
                LauncherState::ReloadConfig => {
                    let config = Config::load();
                    let _ = mem::replace(self, Launcher::new(&config));
                }
                LauncherState::Exit => break,
            }
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

    fn select_application(&mut self) {
        let Some(application) = self.application_list_state.selected() else {
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

    fn handle_input(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => self.input_state.enter_char(to_insert),
                    KeyCode::Backspace => self.input_state.delete_char(),
                    KeyCode::Delete => self.input_state.right_delete_char(),
                    KeyCode::Left => self.input_state.move_cursor_left(),
                    KeyCode::Right => self.input_state.move_cursor_right(),
                    KeyCode::Enter => self.select_application(),
                    KeyCode::Down | KeyCode::Tab => self.application_list_state.select_next(),
                    KeyCode::Up | KeyCode::BackTab => self.application_list_state.select_previous(),
                    KeyCode::Esc => self.state = LauncherState::Exit,
                    KeyCode::F(1) => self.state = LauncherState::ReloadConfig,
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

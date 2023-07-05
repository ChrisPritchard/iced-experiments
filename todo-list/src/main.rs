use iced::alignment::{Vertical, Horizontal};
use iced::widget::container::Appearance;
use iced::{Sandbox, Settings, Element, Length, Theme, Alignment};
use iced::widget::{text_input, row, column, text, container, mouse_area, Text, button};

struct TodoList {
    tasks: Vec<Task>,
    being_edited: Option<u32>,
    being_dragged: Option<u32>,
}

#[derive(Copy, Debug, Clone, PartialEq)]
enum Status {
    Todo, Doing, Done
}

#[derive(Clone, Debug)]
struct Task {
    id: u32,
    description: String,
    status: Status,
}

#[derive(Debug, Clone)]
enum TodoMessage {
    UpdateTask(String),
    EditTask(u32),
    SaveEdited,
    DeleteTask(u32),
    AddTask(Status),
    DraggingTask(u32),
    DroppedInRow(Status)
}

impl Sandbox for TodoList {
    type Message = TodoMessage;

    fn new() -> Self {

        let mut dummy_tasks = Vec::new();
        dummy_tasks.extend([
            Task {
                id: 1, description: "Create a login form: Design a login form with fields for username and password. Implement validation checks and provide appropriate error messages for incorrect credentials.".into(), status: Status::Todo,
            },
            Task {
                id: 2, description: "Build a countdown timer: Develop a countdown timer that starts from a specified time and updates the UI in real-time. Include features like setting the timer duration and displaying a notification when the timer reaches zero.".into(), status: Status::Todo,
            },
            Task {
                id: 3, description: "Implement a draggable element: Create a draggable element that users can click and drag within a defined area. Ensure smooth movement and update the element's position dynamically.".into(), status: Status::Doing,
            },
            Task {
                id: 4, description: "Design a responsive navbar: Develop a responsive navigation bar that adapts to different screen sizes. Include a hamburger menu icon for mobile devices and ensure smooth transitions between menu states.".into(), status: Status::Doing,
            },
            Task {
                id: 5, description: "Add pagination to a data table: Enhance an existing data table by implementing pagination. Display a limited number of rows per page and provide navigation controls to switch between pages.".into(), status: Status::Done,
            },
        ]);

        Self { 
            tasks: dummy_tasks, 
            being_edited: Some(3), 
            being_dragged: None,
        }
    }

    fn title(&self) -> String {
        "To Do List".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            TodoMessage::UpdateTask(content) => {
                self.tasks.iter_mut().for_each(|f| {
                    if f.id == self.being_edited.unwrap() {
                        f.description = content.clone();
                    }
                });
            },
            TodoMessage::EditTask(id) => self.being_edited = Some(id),
            TodoMessage::DeleteTask(id) => {
                if self.being_edited.is_some() && self.being_edited.unwrap() == id {
                    self.being_edited = None;
                }
                self.tasks.retain(|t| t.id != id);
            },
            TodoMessage::AddTask(status) => {
                let next_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                self.tasks.push(Task {
                    id: next_id,
                    description: "".into(),
                    status
                });
                self.being_edited = Some(next_id);
            },
            
            TodoMessage::SaveEdited => self.being_edited = None,
            TodoMessage::DraggingTask(u32) => self.being_dragged = Some(u32),
            TodoMessage::DroppedInRow(new_status) => {
                if self.being_dragged == None {
                    return;
                }
                let id = self.being_dragged.unwrap();
                self.tasks.iter_mut().for_each(|f| {
                    if f.id == id {
                        f.status = new_status;
                    }
                });
            },
        }
    }

    fn view(&self) -> iced::Element<TodoMessage> {

        fn centered_text(content: &str) -> Text {
            text(content.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .vertical_alignment(Vertical::Center)
                .horizontal_alignment(Horizontal::Center)
        }

        fn task_view(task: &Task, is_selected: bool) -> iced::Element<TodoMessage> {
            let style = |theme: &Theme| -> Appearance {
                let palette = theme.extended_palette();
                container::Appearance {
                    border_width: 4.,
                    border_color: palette.primary.strong.color,
                    ..Default::default()
                }
            } as for<'r> fn(&'r _) -> _;

            let content: Element<TodoMessage> = 
                if is_selected {
                    text_input("task description...".into(), &task.description.to_owned())
                        .on_input(TodoMessage::UpdateTask).into()
                } else {
                    text(&task.description).into()
                };
            
            let buttons =
                if is_selected {
                    let save_button = button(centered_text("save")).on_press(TodoMessage::SaveEdited).width(Length::Fill);
                    column(vec![save_button.into()])
                        .align_items(Alignment::Center)
                } else {
                    let edit_button = button(centered_text("edit")).on_press(TodoMessage::EditTask(task.id)).width(Length::Fill);
                    let delete_button = button(centered_text("drop")).on_press(TodoMessage::DeleteTask(task.id)).width(Length::Fill);
                    column(vec![edit_button.into(), delete_button.into()])
                        .align_items(Alignment::Center)
                        .spacing(10)
                };

            container(mouse_area(row(vec![
                    container(content).width(Length::FillPortion(4)).into(),
                    buttons.width(Length::FillPortion(1)).into()
                ])).on_press(TodoMessage::DraggingTask(task.id)))
                .width(Length::Fill)
                .padding(10)
                .style(style)
                .into()
        }

        let selected = self.being_edited.unwrap_or(0);

        fn task_column(this: &TodoList, status: Status, selected: u32) -> Element<TodoMessage> {
            let tasks: Vec<Element<TodoMessage>> = 
                this.tasks
                .iter()
                .filter_map(|t| 
                        if t.status == status { 
                            Some(task_view(t, t.id == selected)) } else { None })
                .collect();
            let heading = match status {
                Status::Todo => "TODO",
                Status::Doing => "DOING",
                Status::Done => "DONE",
            };
            let heading = text(heading).size(30);
            let mut tasks_items: Vec<Element<TodoMessage>> = vec![heading.into()];
            tasks_items.extend(tasks);

            let add_button = button(centered_text("+"))
                .on_press(TodoMessage::AddTask(status))
                .width(Length::Fill);
            tasks_items.push(add_button.into());

            let arrangement =
                column(tasks_items).spacing(10);

            let style = |theme: &Theme| -> Appearance {
                let palette = theme.extended_palette();
                container::Appearance {
                    border_width: 2.,
                    border_color: palette.primary.strong.color,
                    ..Default::default()
                }
            } as for<'r> fn(&'r _) -> _;

            mouse_area(container(arrangement)
                .style(style)
                .width(Length::FillPortion(1))
                .height(Length::Fill)
                .padding(10))
                .on_release(TodoMessage::DroppedInRow(status))
                .into()
        }

        let todo = task_column(self, Status::Todo, selected);
        let doing = task_column(self, Status::Doing, selected);
        let done = task_column(self, Status::Done, selected);

        let content = row(vec![todo, doing, done]).spacing(10);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}

fn main() -> iced::Result {
    TodoList::run(Settings::default())
}

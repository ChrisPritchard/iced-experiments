use iced::{Sandbox, Settings, widget::{text_input, row, column, text, container}, Element, Length, Color, Theme};


struct TodoList {
    tasks: Vec<Task>,
    being_edited: Option<u32>,
}

#[derive(PartialEq)]
enum Status {
    Todo, Doing, Done
}

struct Task {
    id: u32,
    description: String,
    status: Status,
}

#[derive(Debug, Clone)]
enum TodoMessage {
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
        }
    }

    fn title(&self) -> String {
        "To Do List".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<TodoMessage> {

        fn task_view(task: &Task, is_selected: bool) -> iced::Element<TodoMessage> {
            if is_selected {
                let input = text_input("task description...".into(), &task.description.to_owned());
                container(input).into()
            } else {
                let text = text(&task.description);
                container(text).into()
            }
        }

        let selected = self.being_edited.unwrap_or(0);

        pub fn task_column_style(theme: &Theme) -> container::Appearance {
            let palette = theme.extended_palette();
    
            container::Appearance {
                border_width: 2.0,
                border_color: palette.primary.strong.color,
                ..Default::default()
            }
        }

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
            let mut tasks_items: Vec<Element<TodoMessage>> = vec![text(heading).into()];
            tasks_items.extend(tasks);

            let arrangement =
                column(tasks_items)
                    .width(Length::FillPortion(1));

            container(arrangement)
                .style(iced::theme::Container::Custom(task_column_style))
                .into()
        }

        let todo = task_column(self, Status::Todo, selected);
        let doing = task_column(self, Status::Doing, selected);
        let done = task_column(self, Status::Done, selected);

        let content = row(vec![todo, doing, done]);
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(10)
            .into()
    }
}

fn main() -> iced::Result {
    TodoList::run(Settings::default())
}

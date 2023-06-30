use iced::{Sandbox, Settings, widget::{text_input, row, column, text, container}, Element, Length};


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

        let mut todo: Vec<Element<TodoMessage>> = vec![text("TODO").into()];
        todo.extend(self.tasks.iter().filter_map(|t| if t.status == Status::Todo { Some(task_view(t, t.id == selected)) } else { None }));
        let mut doing: Vec<Element<TodoMessage>> = vec![text("DOING").into()];
        doing.extend(self.tasks.iter().filter_map(|t| if t.status == Status::Doing { Some(task_view(t, t.id == selected)) } else { None }));
        let mut done: Vec<Element<TodoMessage>> = vec![text("DONE").into()];
        done.extend(self.tasks.iter().filter_map(|t| if t.status == Status::Done { Some(task_view(t, t.id == selected)) } else { None }));

        let content = 
            row(vec![
                column(todo).width(Length::FillPortion(1)).into(),
                column(doing).width(Length::FillPortion(1)).into(),
                column(done).width(Length::FillPortion(1)).into(),
            ]);

        // three columns, evenly spaced, with headings
        // tasks as boxes within
        // if task is being edited, render with editor
        
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

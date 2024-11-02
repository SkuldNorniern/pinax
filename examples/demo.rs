use pinax::prelude::*;

#[derive(Debug)]
struct MLPrediction {
    height: f64,
    weight: f64,
    raw_pred: f64,
    actual: &'static str,
}

#[derive(Debug)]
struct Student {
    id: &'static str,
    name: &'static str,
    math: u32,
    science: u32,
    english: u32,
}

#[derive(Debug)]
struct Task {
    id: &'static str,
    priority: &'static str,
    description: &'static str,
    status: &'static str,
    due_date: &'static str,
}

fn create_ml_table() -> Table {
    Table::builder()
        .add_column(Column::new("Height").with_alignment(Alignment::Right))
        .add_column(Column::new("Weight").with_alignment(Alignment::Right))
        .add_column(Column::new("Raw Prediction").with_alignment(Alignment::Center))
        .add_column(Column::new("Predicted").with_alignment(Alignment::Center))
        .add_column(Column::new("Actual").with_alignment(Alignment::Center))
        .style(BorderStyle::Double)
        .build()
}

fn create_student_table() -> Table {
    Table::builder()
        .add_column(Column::new("Student ID").with_alignment(Alignment::Center))
        .add_column(Column::new("Name").with_alignment(Alignment::Left))
        .add_column(Column::new("Math").with_alignment(Alignment::Right))
        .add_column(Column::new("Science").with_alignment(Alignment::Right))
        .add_column(Column::new("English").with_alignment(Alignment::Right))
        .add_column(Column::new("Average").with_alignment(Alignment::Right))
        .style(BorderStyle::Rounded)
        .build()
}

fn create_task_table() -> Table {
    Table::builder()
        .add_column(Column::new("Task ID").with_alignment(Alignment::Center))
        .add_column(Column::new("Priority").with_alignment(Alignment::Center))
        .add_column(Column::new("Description").with_alignment(Alignment::Left))
        .add_column(Column::new("Status").with_alignment(Alignment::Center))
        .add_column(Column::new("Due Date").with_alignment(Alignment::Center))
        .style(BorderStyle::Single)
        .build()
}

fn print_ml_predictions() {
    let predictions = vec![
        MLPrediction {
            height: 184.3,
            weight: 89.5,
            raw_pred: 0.0000,
            actual: "Male",
        },
        MLPrediction {
            height: 176.8,
            weight: 79.2,
            raw_pred: 0.0036,
            actual: "Male",
        },
        MLPrediction {
            height: 165.9,
            weight: 58.7,
            raw_pred: 0.9214,
            actual: "Female",
        },
        MLPrediction {
            height: 164.1,
            weight: 57.8,
            raw_pred: 0.9441,
            actual: "Female",
        },
        MLPrediction {
            height: 181.9,
            weight: 85.8,
            raw_pred: 0.0001,
            actual: "Male",
        },
        MLPrediction {
            height: 157.9,
            weight: 50.6,
            raw_pred: 0.9893,
            actual: "Female",
        },
        MLPrediction {
            height: 178.4,
            weight: 81.6,
            raw_pred: 0.0010,
            actual: "Male",
        },
    ];

    let mut table = create_ml_table();

    for pred in predictions {
        let predicted = if pred.raw_pred > 0.5 {
            "Female"
        } else {
            "Male"
        };
        table.add_row(vec![
            format!("{:.1}", pred.height),
            format!("{:.1}", pred.weight),
            format!("{:.4}", pred.raw_pred),
            predicted.to_string(),
            pred.actual.to_string(),
        ]);
    }

    println!("\n📊 Machine Learning Gender Predictions");
    println!("=====================================");
    println!("{}", table);
}

fn print_student_grades() {
    let students = vec![
        Student {
            id: "2024001",
            name: "Alice Johnson",
            math: 95,
            science: 92,
            english: 88,
        },
        Student {
            id: "2024002",
            name: "Bob Smith",
            math: 88,
            science: 90,
            english: 85,
        },
        Student {
            id: "2024003",
            name: "Carol Williams",
            math: 92,
            science: 95,
            english: 94,
        },
        Student {
            id: "2024004",
            name: "David Brown",
            math: 78,
            science: 82,
            english: 80,
        },
        Student {
            id: "2024005",
            name: "Emma Davis",
            math: 98,
            science: 96,
            english: 92,
        },
    ];

    let mut table = create_student_table();

    for student in students {
        let avg = (student.math + student.science + student.english) as f64 / 3.0;
        table.add_row(vec![
            student.id.to_string(),
            student.name.to_string(),
            student.math.to_string(),
            student.science.to_string(),
            student.english.to_string(),
            format!("{:.1}", avg),
        ]);
    }

    println!("\n📚 Student Grade Report");
    println!("=====================");
    println!("{}", table);
}

fn print_project_status() {
    let tasks = vec![
        Task {
            id: "TASK-01",
            priority: "HIGH",
            description: "Implement user authentication",
            status: "In Progress",
            due_date: "2024-04-01",
        },
        Task {
            id: "TASK-02",
            priority: "MEDIUM",
            description: "Design database schema",
            status: "Completed",
            due_date: "2024-03-25",
        },
        Task {
            id: "TASK-03",
            priority: "HIGH",
            description: "API documentation",
            status: "Pending",
            due_date: "2024-04-05",
        },
        Task {
            id: "TASK-04",
            priority: "LOW",
            description: "Unit test coverage",
            status: "Not Started",
            due_date: "2024-04-10",
        },
        Task {
            id: "TASK-05",
            priority: "MEDIUM",
            description: "Performance optimization",
            status: "In Progress",
            due_date: "2024-04-15",
        },
    ];

    let mut table = create_task_table();

    for task in tasks {
        table.add_row(vec![
            task.id.to_string(),
            task.priority.to_string(),
            task.description.to_string(),
            task.status.to_string(),
            task.due_date.to_string(),
        ]);
    }

    println!("\n📋 Project Task Status");
    println!("====================");
    println!("{}", table);
}

fn print_grid_demo() {
    // Simple 3x3 Grid Example
    let mut simple_grid = Grid::builder()
        .dimensions(3, 3)
        .style(BorderStyle::Single)
        .build();

    simple_grid.set(0, 0, "1");
    simple_grid.set(0, 1, "2");
    simple_grid.set(0, 2, "3");
    simple_grid.set(1, 1, "Center");
    simple_grid.set(2, 0, "7");
    simple_grid.set(2, 1, "8");
    simple_grid.set(2, 2, "9");

    println!("\n📱 Simple 3x3 Grid");
    println!("================");
    println!("{}", simple_grid);

    // Calendar-like Grid
    let mut calendar = Grid::builder()
        .dimensions(5, 7)
        .style(BorderStyle::Double)
        .build();

    // Headers
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for (i, day) in days.iter().enumerate() {
        calendar.set(0, i, day.to_string());
    }

    // Calendar dates
    let dates = [
        ["1", "2", "3", "4", "5", "6", "7"],
        ["8", "9", "10", "11", "12", "13", "14"],
        ["15", "16", "17", "18", "19", "20", "21"],
        ["22", "23", "24", "25", "26", "27", "28"],
    ];

    for (row, week) in dates.iter().enumerate() {
        for (col, date) in week.iter().enumerate() {
            calendar.set(row + 1, col, date.to_string());
        }
    }

    println!("\n📅 Calendar Grid Example");
    println!("=====================");
    println!("{}", calendar);
}

fn print_panel_demo() {
    let welcome_panel = Panel::new(
        "Welcome to Pinax - a powerful terminal UI toolkit for Rust! \
        This panel demonstrates text wrapping and border styling capabilities. \
        Long text will automatically wrap to fit within the specified width.",
    )
    .with_title("Welcome")
    .with_width(60);

    let stats_panel = Panel::new(
        "📈 Active Users: 1,234\n\
         🔄 Server Uptime: 99.9%\n\
         💾 Memory Usage: 2.1GB\n\
         🚀 Response Time: 45ms",
    )
    .with_title("System Stats")
    .with_width(30);

    println!("\n📦 Panel Examples");
    println!("===============");
    println!("{}", welcome_panel);
    println!("\n{}", stats_panel);
}

fn print_chart_demo() {
    // Line Chart Example
    let mut line_chart = Chart::new(ChartType::Line)
        .with_height(15)
        .with_title("Monthly Revenue (in $K)");

    line_chart.add_data_point("Jan", 10.0);
    line_chart.add_data_point("Feb", 15.0);
    line_chart.add_data_point("Mar", 13.0);
    line_chart.add_data_point("Apr", 17.0);
    line_chart.add_data_point("May", 20.0);
    line_chart.add_data_point("Jun", 18.0);

    println!("\n{}", line_chart);

    // Bar Chart Example
    let mut bar_chart = Chart::new(ChartType::Bar)
        .with_title("Daily Activity (Tasks Completed)");
        
    bar_chart.add_data_point("Mon", 45.0);
    bar_chart.add_data_point("Tue", 30.0);
    bar_chart.add_data_point("Wed", 60.0);
    bar_chart.add_data_point("Thu", 35.0);
    bar_chart.add_data_point("Fri", 50.0);

    println!("\n{}", bar_chart);
}

fn main() {
    println!("🌟 Pinax Table Formatting Demo 🌟");
    println!("================================");

    print_ml_predictions();
    println!();

    print_student_grades();
    println!();

    print_project_status();
    println!();

    print_grid_demo();
    println!();

    // Add the new demo functions
    print_panel_demo();
    println!();

    print_chart_demo();
}

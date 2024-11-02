# Pinax

> **Pinax** (πίναξ) is a Greek word meaning "board," "tablet," or "list."

**Pinax** is a lightweight library for printing beautifully formatted tables in your terminal applications. Designed to make data presentation clean and straightforward, Pinax helps you display structured data in an elegant and readable format.

## Features

- **Easy Table Creation**: Quickly generate tables with minimal code.
- **Customizable Borders**: Choose from different border styles to suit your preferences.
- **Automatic Column Sizing**: Adjusts column widths based on content for optimal display.
- **Alignment Options**: Align text to the left, right, or center within columns.
- **Unicode Support**: Utilize Unicode characters to enhance your tables.

*Note: In the initial release, Pinax focuses on table creation and formatting. Future updates may introduce additional features to expand its capabilities.*

## Preview

Here's how different components look in action:

### Tables and Grids
```
📊 Machine Learning Gender Predictions
=====================================
╔══════════╦══════════╦══════════════════╦═════════════╦══════════╗
║ Height   ║ Weight   ║ Raw Prediction   ║ Predicted   ║ Actual   ║
╠══════════╬══════════╬══════════════════╬═════════════╬══════════╣
║  184.3   ║   89.5   ║     0.0000       ║   Male      ║  Male    ║
║  176.8   ║   79.2   ║     0.0036       ║   Male      ║  Male    ║
║  165.9   ║   58.7   ║     0.9214       ║  Female     ║ Female   ║
╚══════════╩══════════╩══════════════════╩═════════════╩══════════╝
```

### Panels with Emoji Support
```
┌──────[ System Stats ]──────┐
│ 📈 Active Users: 1,234     │
│ 🔄 Server Uptime: 99.9%    │
│ 💾 Memory Usage: 2.1GB     │
│ 🚀 Response Time: 45ms     │
└────────────────────────────┘
```

### Charts
```
Monthly Revenue (in $K)
=======================

    20.0  │                             ●\\                    
          │                           //   \\                
          │                          /        \\            
    15.0  │           ●\\\         /            ●           
          │         /     \\\   //                               
          │      ///         ●//                                 
    10.0  │ ●///                                                                      
          └──────────────────────────────────────────────────
            Jan      Feb      Mar      Apr      May      
```

### Calendar Grid
```
╔════════════════╦════════════════╦════════════════╗
║      Sun       ║      Mon       ║      Tue       ║
╠════════════════╬════════════════╬════════════════╣
║       1        ║       2        ║       3        ║
╠════════════════╬════════════════╬════════════════╣
║       8        ║       9        ║       10       ║
╚════════════════╩════════════════╩════════════════╝
```

## Example

```rust
use pinax::prelude::*;

fn main() {
    let headers = vec!["Height", "Weight", "Raw Prediction", "Predicted", "Actual"];
    let data = vec![
        vec!["184.3", "89.5", "0.0000", "Male", "Male"],
        vec!["176.8", "79.2", "0.0036", "Male", "Male"],
        vec!["181.7", "86.9", "0.0001", "Male", "Male"],
        // Add more rows as needed
    ];

    let mut table = Table::new();
    table.set_headers(headers);
    table.add_rows(data);
    table.print();
}
```

This will output:

```
╔══════════╤══════════╤═════════════════╤═══════════╤══════════╗
║  Height  │  Weight  │ Raw Prediction  │ Predicted │  Actual  ║
╠══════════╪══════════╪═════════════════╪═══════════╪══════════╣
║  184.3   │   89.5   │     0.0000      │   Male    │   Male   ║
║  176.8   │   79.2   │     0.0036      │   Male    │   Male   ║
║  181.7   │   86.9   │     0.0001      │   Male    │   Male   ║
╚══════════╧══════════╧═════════════════╧═══════════╧══════════╝
```

## Installation

Add Pinax to your `Cargo.toml` dependencies:

```toml
[dependencies]
pinax = "0.1.0"
```

Then import it into your project:

```rust
use pinax::prelude::*;
```

## Roadmap

While the initial release of Pinax focuses on table creation and formatting, we have plans to expand its functionality in future updates. Potential features include:

- **Additional Data Structures**: Support for grids, matrices, and other structured formats.
- **Styling Options**: More control over colors, fonts, and styles.
- **Interactive Tables**: Features like sortable columns or selectable rows.
- **Export Formats**: Ability to export tables to formats like CSV, JSON, or HTML.



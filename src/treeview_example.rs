//! # TreeView Sample
//!
//! This sample demonstrates how to create a `TreeView` with either a `ListStore` or `TreeStore`.
extern crate gio;
extern crate gtk;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib;
use gtk::prelude::*;
use gtk::{
     ApplicationWindow, ButtonsType, CellRendererPixbuf, CellRendererText, DialogFlags,
    MessageDialog, MessageType, Orientation, TreeStore, TreeView, TreeViewColumn, WindowPosition,
};

use std::env::args;

fn append_text_column(tree: &TreeView) {
    println!("1 append text column start");

    let column = TreeViewColumn::new(); // TreeViewColumn { inner: ObjectRef { inner: 0xa4ac2e0, type: GtkTreeViewColumn } }
    println!("2 column: {:?}", column);

    let cell = CellRendererText::new(); //CellRendererText { inner: ObjectRef { inner: 0xa4f2210, type: GtkCellRendererText } }
    println!("3 cell: {:?}", cell);

    let start_result = column.pack_start(&cell, true); // ()
    println!("4 column pack start: {:?}", start_result);

    let add_result = column.add_attribute(&cell, "text", 0); // ()
    println!("5 add attribute: {:?}", add_result);

    let append_column_result = tree.append_column(&column); // 1
    println!("6 append column: {:?}", append_column_result);
}

fn build_ui(application: &gtk::Application) {
    println!("1 build ui start");

    let window = ApplicationWindow::new(application); //  ApplicationWindow { inner: ObjectRef { inner: 0xa56f740, type: GtkApplicationWindow } }
    println!("2 window: {:?}", window);

    window.set_title("TreeView Sample");
    window.set_position(WindowPosition::Center);
    println!("3 set title and position");

    // left pane
    let left_tree = TreeView::new(); // TreeView { inner: ObjectRef { inner: 0xbcc1df0, type: GtkTreeView } }
    println!("4 left_tree: {:?}", left_tree);

    let left_store = TreeStore::new(&[String::static_type()]); // TreeStore { inner: ObjectRef { inner: 0xbe087c0, type: GtkTreeStore } }
    println!("5 left_store: {:?}", left_store);

    let result1 = left_tree.set_model(Some(&left_store)); // ()
    println!("6 set model: {:?}", result1);
    let result2 = left_tree.set_headers_visible(false); // ()
    println!("7 set header visible false: {:?}", result2);
    let result3 = append_text_column(&left_tree); // ()
    println!("8 append left column: {:?}", result3);

    for i in 0..10 {
        println!("9 in for loop");

        // insert_with_values takes a slice of tuples: column index and ToValue
        // trait objects. ToValue is implemented for strings, numeric types,
        // bool and Object descendants
        let iter = left_store.insert_with_values(None, None, &[0], &[(&format!("Hello {}", i))]); // iter: TreeIter(Boxed { inner: Native(0x8784ce0) })
        println!("10 iter: {:?}", iter);
        for _ in 0..i {
            let result =
                left_store.insert_with_values(Some(&iter), None, &[0], &[(&"I'm a child node")]);
            println!("11 child node: {:?}", result); // TreeIter(Boxed { inner: Native(0x87843e0) })
        }
    }
    println!("11 for loop exit");

    // right pane
    let right_tree = TreeView::new();
    let right_column_types = [Pixbuf::static_type(), String::static_type()];
    let right_store = TreeStore::new(&right_column_types);
    let renderer = CellRendererPixbuf::new();
    let col = TreeViewColumn::new();
    println!("12");

    col.set_title("Picture");
    col.pack_start(&renderer, false);
    println!("13");

    col.add_attribute(&renderer, "pixbuf", 0);
    println!("14");

    let renderer2 = CellRendererText::new();
    col.pack_start(&renderer2, true);

    col.add_attribute(&renderer2, "text", 1);
    println!("15");

    let image = Pixbuf::from_resource("./eye.png")
        .map_err(|err| {
            let msg = err.to_string();
            glib::idle_add_local(
                glib::clone!(@weak window => @default-return glib::Continue(false), move || {
                    let dialog = MessageDialog::new(Some(&window), DialogFlags::MODAL,
                        MessageType::Error, ButtonsType::Ok, &msg);
                    dialog.connect_response(|dialog, _| dialog.close());
                    dialog.show_all();
                    Continue(false)
                }),
            );
        })
        .ok();
    println!("16 image: {:?}", image);

    right_tree.append_column(&col);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);
    println!("17");

    for _ in 0..10 {
        println!("18");
        // insert_with_values(parent, position, columns, values)
        
        // Parameters:
        // parent (Gtk.TreeIter or None) – A valid Gtk.TreeIter, or None (Option<&TreeIter>)
        // position (int) – position to insert the new row, or -1 for last (Option<u32>)
        // columns ([int]) – an array of column numbers (&[u32])
        // values ([GObject.Value]) – an array of GValues (&[&dyn ToValue])

        // Returns: An unset Gtk.TreeIter to set the new row, or None.
        right_store.insert_with_values(
            None,
            None,
            &[0,1],
            &[(&image), (&"I'm a child node with an image")],
        );
    }
    println!("19");

    // selection and path manipulation

    let left_selection = left_tree.get_selection();
    println!("16 build ui");

    left_selection.connect_changed(glib::clone!(@weak right_tree => move |tree_selection| {
        println!("17 build ui");

        let (left_model, iter) = tree_selection.get_selected().expect("Couldn't get selected");
        let mut path = left_model.get_path(&iter).expect("Couldn't get path");
        // get the top-level element path
        while path.get_depth() > 1 {
            path.up();
        }
        right_tree.get_selection().select_path(&path);
    }));

    // display the panes
    println!("18 build ui");

    let split_pane = gtk::Box::new(Orientation::Horizontal, 10);
    println!("19 build ui");

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);
    println!("20 build ui");

    window.add(&split_pane);
    println!("21 build ui");

    window.show_all();
}

pub fn start_treeview() {
    println!("1 start tree view");

    // gio::resources_register_include!("compiled.gresource").unwrap(); // not work. looks like it runs without this for now
    println!("2 start tree view");

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.treeview"),
        Default::default(),
    )
    .expect("Initialization failed...");
    // application: Application { inner: ObjectRef { inner: 0xa4a9150, type: GtkApplication } }
    println!("3 start tree view: {:?}", application);

    let result = application.connect_activate(|app| {
        println!("4 start tree view: {:?}", app);
        // app: Application { inner: ObjectRef { inner: 0xa4a9150, type: GtkApplication } }
        build_ui(app);
    }); // SignalHandlerId(6)
    println!("5 start tree view: {:?}", result);

    let result_2 = application.run(&args().collect::<Vec<_>>());
    println!("6 start tree view: {:?}", result_2);  // 0 (when the app window exits)
}

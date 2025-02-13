use crate::util::create_folder;
use collab_folder::core::{Belonging, Belongings, Workspace};

#[test]
fn create_workspace_test() {
    let folder_test = create_folder("1");

    let belongings = Belongings {
        items: vec![
            Belonging::new("1".to_string()),
            Belonging::new("2".to_string()),
        ],
    };
    let o_workspace = Workspace {
        id: "1".to_string(),
        name: "My first workspace".to_string(),
        belongings,
        created_at: 123,
    };

    folder_test.workspaces.create_workspace(o_workspace.clone());
    let r_workspace = folder_test.workspaces.get_all_workspaces().remove(0);

    assert_eq!(o_workspace.name, r_workspace.name);
    assert_eq!(o_workspace.id, r_workspace.id);
    assert_eq!(o_workspace.belongings, r_workspace.belongings);
}

#[test]
fn update_workspace_test() {
    let folder_test = create_folder("1");
    let workspace = Workspace {
        id: "1".to_string(),
        name: "My first workspace".to_string(),
        belongings: Belongings {
            items: vec![
                Belonging::new("1".to_string()),
                Belonging::new("2".to_string()),
            ],
        },
        created_at: 123,
    };

    folder_test.workspaces.create_workspace(workspace);
    let workspace_map = folder_test.workspaces.edit_workspace("1").unwrap();
    workspace_map.update(|update| {
        update.set_name("New workspace").delete_belongings(0);
    });

    // folder_test.workspaces.
    let workspace = folder_test.workspaces.get_workspace("1").unwrap();
    assert_eq!(workspace.name, "New workspace");
    assert_eq!(workspace.belongings.len(), 1);
    assert_eq!(workspace.belongings[0].id, "2");
}

#[test]
fn get_all_workspace_test() {
    let folder_test = create_folder("1");
    for i in 0..10 {
        let workspace = Workspace {
            id: i.to_string(),
            name: format!("My {} workspace", i),
            belongings: Default::default(),
            created_at: 123,
        };
        folder_test.workspaces.create_workspace(workspace);
    }

    let workspaces = folder_test.workspaces.get_all_workspaces();
    assert_eq!(workspaces.len(), 10);
}

#[test]
fn delete_workspace_test() {
    let folder_test = create_folder("1");
    for i in 0..10 {
        let workspace = Workspace {
            id: i.to_string(),
            name: format!("My {} workspace", i),
            belongings: Default::default(),
            created_at: 123,
        };
        folder_test.workspaces.create_workspace(workspace);
    }

    folder_test.workspaces.delete_workspace(0);
    let workspaces = folder_test.workspaces.get_all_workspaces();
    assert_eq!(workspaces.len(), 9);
    assert_eq!(workspaces[0].id, "1");
}

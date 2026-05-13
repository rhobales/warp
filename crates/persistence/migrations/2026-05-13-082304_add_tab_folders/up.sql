CREATE TABLE tab_folders (
    id INTEGER PRIMARY KEY NOT NULL,
    window_id INTEGER NOT NULL,
    local_folder_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    color TEXT,
    is_open BOOLEAN NOT NULL DEFAULT 1,
    sidebar_position INTEGER NOT NULL,
    FOREIGN KEY (window_id) REFERENCES windows(id),
    UNIQUE (window_id, local_folder_id)
);

ALTER TABLE tabs ADD COLUMN local_tab_id INTEGER;
ALTER TABLE tabs ADD COLUMN folder_id INTEGER REFERENCES tab_folders(id);
ALTER TABLE tabs ADD COLUMN sidebar_position INTEGER NOT NULL DEFAULT 0;

UPDATE tabs SET sidebar_position = (
    SELECT COUNT(*) FROM tabs t2
    WHERE t2.window_id = tabs.window_id AND t2.id <= tabs.id
) - 1;

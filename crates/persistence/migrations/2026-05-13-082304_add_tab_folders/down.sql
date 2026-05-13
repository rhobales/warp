CREATE TABLE tabs_old (
    id INTEGER PRIMARY KEY NOT NULL,
    window_id INTEGER NOT NULL,
    custom_title TEXT,
    color TEXT,
    FOREIGN KEY (window_id) REFERENCES windows(id)
);

INSERT INTO tabs_old (id, window_id, custom_title, color)
SELECT id, window_id, custom_title, color FROM tabs;

DROP TABLE tabs;
ALTER TABLE tabs_old RENAME TO tabs;

DROP TABLE tab_folders;

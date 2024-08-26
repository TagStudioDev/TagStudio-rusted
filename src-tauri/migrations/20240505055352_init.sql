CREATE TABLE location (
    `id` integer PRIMARY KEY,
    `path` text NOT NULL UNIQUE,
    `name` text
) strict;

CREATE TABLE `entry` (
    `id` integer PRIMARY KEY,
    `path` text NOT NULL,
    `hash` blob NOT NULL
) strict;

CREATE TABLE `color` (
    `id` integer PRIMARY KEY,
    `name` text NOT NULL,
    `primary` integer NOT NULL DEFAULT 0,
    `accent` integer,
    `border` integer
) strict;

CREATE TABLE `tag` (
    `id` integer PRIMARY KEY,
    `name` text NOT NULL,
    `color` integer NOT NULL REFERENCES `color` (`id`),
    `icon` text,
    `preference` integer REFERENCES `tag` (`id`),
    `field` integer NOT NULL DEFAULT 0,
    `system` integer NOT NULL DEFAULT 0
) strict;

CREATE TABLE `tag_relation` (
    `tag` integer NOT NULL REFERENCES `tag` (`id`),
    `parent_tag` integer NOT NULL REFERENCES `tag` (`id`),
    `negate` integer NOT NULL DEFAULT 0,
    PRIMARY KEY (`tag`, `parent_tag`)
) strict,
WITHOUT rowid;

CREATE TABLE `entry_attribute` (
    `entry` integer NOT NULL REFERENCES `entry` (`id`),
    `field` integer NOT NULL REFERENCES `tag` (`id`),
    `tag` integer REFERENCES `tag` (`id`),
    `text` text,
    `number` real,
    PRIMARY KEY (`entry`, `field`)
) strict,
WITHOUT rowid;

CREATE TABLE `entry_page` (
    `entry` integer NOT NULL REFERENCES `entry` (`id`),
    `tag` integer NOT NULL REFERENCES `tag` (`id`),
    `page` integer,
    PRIMARY KEY (`entry`, `page`)
) strict,
WITHOUT rowid;

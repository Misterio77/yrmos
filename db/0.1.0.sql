BEGIN;

CREATE TABLE person (
    id uuid NOT NULL,
    email varchar NOT NULL,
    password varchar NOT NULL,
    real_name varchar NOT NULL,
    pix_key varchar,

    CONSTRAINT person_pk PRIMARY KEY (id),
    CONSTRAINT person_email_un UNIQUE (email)
);

CREATE TABLE session (
    id uuid NOT NULL,
    creator_id uuid NOT NULL,
    creation timestamptz NOT NULL,

    CONSTRAINT session_pk PRIMARY KEY (id),
    CONSTRAINT session_creator_fk FOREIGN KEY (creator_id)
        REFERENCES person (id) ON DELETE CASCADE
);

CREATE TABLE ride (
    id uuid NOT NULL,
    driver varchar NOT NULL,
    seats int NOT NULL DEFAULT 4,
    departure timestamptz NOT NULL,
    start_location varchar NOT NULL,
    end_location varchar NOT NULL,
    cost DECIMAL DEFAULT NULL,

    CONSTRAINT ride_pk PRIMARY KEY (id)
);

CREATE TABLE rider (
    ride_id uuid NOT NULL,
    person_id uuid NOT NULL,
    review boolean,

    CONSTRAINT rider_pk PRIMARY KEY (ride_id, person_id),
    CONSTRAINT rider_person_fk FOREIGN KEY (person_id)
        REFERENCES person (id) ON DELETE CASCADE,
    CONSTRAINT rider_ride_fk FOREIGN KEY (ride_id)
        REFERENCES ride (id) ON DELETE CASCADE
);

COMMIT;

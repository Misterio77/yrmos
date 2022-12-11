BEGIN;

CREATE TABLE person (
    email varchar NOT NULL,
    password varchar NOT NULL,
    real_name varchar NOT NULL,
    pix_key varchar,

    CONSTRAINT person_pk PRIMARY KEY (email)
);

CREATE TABLE session (
    id uuid NOT NULL,
    creator varchar NOT NULL,
    creation timestamptz NOT NULL,

    CONSTRAINT session_pk PRIMARY KEY (id),
    CONSTRAINT session_creator_fk FOREIGN KEY (creator)
        REFERENCES person (email) ON DELETE CASCADE
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
    ride uuid NOT NULL,
    person varchar NOT NULL,
    review boolean,

    CONSTRAINT rider_pk PRIMARY KEY (ride, person),
    CONSTRAINT rider_person_fk FOREIGN KEY (person)
        REFERENCES person (email) ON DELETE CASCADE,
    CONSTRAINT rider_ride_fk FOREIGN KEY (ride)
        REFERENCES ride (id) ON DELETE CASCADE
);

COMMIT;
BEGIN;

ALTER TABLE rider
    ADD COLUMN rate boolean;

CREATE VIEW reputation AS
    SELECT person.email AS person,
        (
            COUNT(CASE WHEN rider.rate THEN 1 END) -
            COUNT(CASE WHEN (NOT rider.rate) THEN 1 END)
        ) AS score
    FROM person
    INNER JOIN ride ON ride.driver = person.email
    INNER JOIN rider ON rider.ride = ride.id
    WHERE rate IS NOT NULL
    GROUP BY person.email;

COMMIT;

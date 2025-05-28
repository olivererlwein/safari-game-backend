
-- Tabla de jugadores (sin autenticación compleja)
CREATE TABLE players (
    player_id VARCHAR(36) PRIMARY KEY,
    player_name VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Tabla de mundos/hábitats (datos fijos)
CREATE TABLE worlds (
    world_id TINYINT PRIMARY KEY,  -- 1: Sabana, 2: Bolivia
    world_name VARCHAR(20) NOT NULL
);

-- Tabla de progreso (el núcleo del sistema)
CREATE TABLE player_progress (
    progress_id INT AUTO_INCREMENT PRIMARY KEY,
    player_id VARCHAR(36) NOT NULL,
    world_id TINYINT NOT NULL,
    current_level TINYINT DEFAULT 1,  -- Nivel actual alcanzado (1-3)
    levels_completed TINYINT DEFAULT 0, -- Niveles completados (0-3)
    quiz_score SMALLINT DEFAULT 0,    -- Puntaje acumulado
    last_played TIMESTAMP NULL,      -- Fecha último intento
    UNIQUE KEY (player_id, world_id), -- Evita duplicados
    FOREIGN KEY (player_id) REFERENCES players(player_id),
    FOREIGN KEY (world_id) REFERENCES worlds(world_id)
);
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

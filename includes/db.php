<?php
/**
 * Połączenie z bazą danych (PDO)
 * Używa ustawień z config.php
 */

/**
 * Zwraca pojedyncze połączenie PDO
 * @return PDO
 */
function get_db_connection() {
    static $db = null;
    
    if ($db === null) {
        try {
            $dsn = 'mysql:host=' . DB_HOST . ';dbname=' . DB_NAME . ';charset=' . DB_CHARSET;
            $options = [
                PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
                PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
                PDO::ATTR_EMULATE_PREPARES => false,
                PDO::ATTR_PERSISTENT => true,
            ];
            
            $db = new PDO($dsn, DB_USER, DB_PASS, $options);
        } catch (PDOException $e) {
            // W trybie debug wyświetl błąd
            if (DEBUG) {
                die('Błąd połączenia z bazą danych: ' . $e->getMessage());
            } else {
                die('Wystąpił błąd połączenia z bazą danych. Proszę spróbować później.');
            }
        }
    }
    
    return $db;
}

/**
 * Wykonuje zapytanie SELECT i zwraca wyniki
 * @param string $sql
 * @param array $params
 * @return array
 */
function db_select($sql, $params = []) {
    $db = get_db_connection();
    $stmt = $db->prepare($sql);
    $stmt->execute($params);
    return $stmt->fetchAll(PDO::FETCH_ASSOC);
}

/**
 * Wykonuje zapytanie SELECT i zwraca pojedynczy wiersz
 * @param string $sql
 * @param array $params
 * @return array|null
 */
function db_select_one($sql, $params = []) {
    $db = get_db_connection();
    $stmt = $db->prepare($sql);
    $stmt->execute($params);
    return $stmt->fetch(PDO::FETCH_ASSOC);
}

/**
 * Wykonuje zapytanie INSERT/UPDATE/DELETE
 * @param string $sql
 * @param array $params
 * @return int (liczba zmienionych wierszy)
 */
function db_execute($sql, $params = []) {
    $db = get_db_connection();
    $stmt = $db->prepare($sql);
    $stmt->execute($params);
    return $stmt->rowCount();
}

/**
 * Zwraca ostatnie ID wstawionego wiersza
 * @return string
 */
function db_last_insert_id() {
    $db = get_db_connection();
    return $db->lastInsertId();
}

/**
 * Rozpoczyna transakcję
 */
function db_begin_transaction() {
    get_db_connection()->beginTransaction();
}

/**
 * Potwierdza transakcję
 */
function db_commit() {
    get_db_connection()->commit();
}

/**
 * Wycofa transakcję
 */
function db_rollback() {
    get_db_connection()->rollBack();
}

/**
 * Sprawdza czy tabela istnieje
 * @param string $tableName
 * @return bool
 */
function table_exists($tableName) {
    $db = get_db_connection();
    $stmt = $db->prepare("SHOW TABLES LIKE ?");
    $stmt->execute([$tableName]);
    return $stmt->rowCount() > 0;
}

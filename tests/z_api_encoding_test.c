#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "zenoh.h"

#undef NDEBUG
#include <assert.h>

#define assert_str_eq(expected, actual_str)                                     \
    do {                                                                        \
        const char* _expected = (expected);                                     \
        const z_loaned_string_t* _actual = (actual_str);                        \
        size_t _expected_len = strlen(_expected);                               \
        assert(z_string_len(_actual) == _expected_len);                         \
        assert(strncmp(_expected, z_string_data(_actual), _expected_len) == 0); \
    } while (0)

void test_null_encoding(void) {
    z_owned_encoding_t e;
    z_internal_encoding_null(&e);
    assert(!z_internal_encoding_check(&e));
    z_encoding_drop(z_move(e));
}

void test_encoding_without_id(void) {
    z_owned_encoding_t e1;
    z_encoding_from_str(&e1, "my_encoding");
    assert(z_internal_encoding_check(&e1));
    z_owned_string_t s;
    z_encoding_to_string(z_encoding_loan(&e1), &s);
    assert_str_eq("my_encoding", z_string_loan(&s));
    z_encoding_drop(z_move(e1));
    z_string_drop(z_move(s));

    z_owned_encoding_t e2;
    z_encoding_from_substr(&e2, "my_encoding", 4);
    assert(z_internal_encoding_check(&e2));

    z_encoding_to_string(z_encoding_loan(&e2), &s);
    assert_str_eq("my_e", z_string_loan(&s));
    z_encoding_drop(z_move(e2));
    z_string_drop(z_move(s));
}

void test_encoding_with_id(void) {
    z_owned_encoding_t e1;
    z_encoding_from_str(&e1, "zenoh/string;utf8");
    assert(z_internal_encoding_check(&e1));
    z_owned_string_t s;
    z_encoding_to_string(z_encoding_loan(&e1), &s);
    assert_str_eq("zenoh/string;utf8", z_string_loan(&s));
    z_encoding_drop(z_move(e1));
    z_string_drop(z_move(s));

    z_owned_encoding_t e2;
    z_encoding_from_substr(&e2, "zenoh/string;utf8", 17);
    assert(z_internal_encoding_check(&e2));

    z_encoding_to_string(z_encoding_loan(&e2), &s);
    assert_str_eq("zenoh/string;utf8", z_string_loan(&s));
    z_encoding_drop(z_move(e2));
    z_string_drop(z_move(s));

    z_owned_encoding_t e3;
    z_encoding_from_str(&e3, "custom_id;custom_schema");
    assert(z_internal_encoding_check(&e3));

    z_encoding_to_string(z_encoding_loan(&e3), &s);
    assert_str_eq("custom_id;custom_schema", z_string_loan(&s));
    z_encoding_drop(z_move(e3));
    z_string_drop(z_move(s));

    z_owned_encoding_t e4;
    z_encoding_from_substr(&e4, "custom_id;custom_schema", 16);
    assert(z_internal_encoding_check(&e4));

    z_encoding_to_string(z_encoding_loan(&e4), &s);
    assert_str_eq("custom_id;custom", z_string_loan(&s));
    z_encoding_drop(z_move(e4));
    z_string_drop(z_move(s));
}

void test_with_schema(void) {
    z_owned_encoding_t e;
    z_internal_encoding_null(&e);
    z_encoding_set_schema_from_str(z_encoding_loan_mut(&e), "my_schema");

    z_owned_string_t s;
    z_encoding_to_string(z_encoding_loan(&e), &s);

    assert_str_eq("zenoh/bytes;my_schema", z_string_loan(&s));
    z_encoding_drop(z_move(e));

    z_encoding_clone(&e, z_encoding_zenoh_string());
    z_encoding_set_schema_from_substr(z_encoding_loan_mut(&e), "my_schema", 3);

    z_encoding_to_string(z_encoding_loan(&e), &s);
    assert_str_eq("zenoh/string;my_", z_string_loan(&s));
    z_encoding_drop(z_move(e));
    z_string_drop(z_move(s));
}

void test_constants() {
    z_owned_string_t s;
    z_encoding_to_string(z_encoding_zenoh_bytes(), &s);
    assert_str_eq("zenoh/bytes", z_string_loan(&s));
    z_string_drop(z_move(s));

    z_encoding_to_string(z_encoding_zenoh_string(), &s);
    assert_str_eq("zenoh/string", z_string_loan(&s));

    z_string_drop(z_move(s));
}

void test_equals() {
    z_owned_encoding_t e;
    z_encoding_from_str(&e, "zenoh/string");
    assert(z_encoding_equals(z_loan(e), z_encoding_zenoh_string()));
    assert(!z_encoding_equals(z_loan(e), z_encoding_zenoh_serialized()));
    z_drop(z_move(e));
}

int main(int argc, char** argv) {
    test_null_encoding();
    test_encoding_without_id();
    test_encoding_with_id();
    test_constants();
    test_with_schema();
    test_equals();
}

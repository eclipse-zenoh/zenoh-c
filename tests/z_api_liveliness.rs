//
// Copyright (c) 2025 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

use std::{ffi::c_void, mem::MaybeUninit, thread::sleep, time::Duration};

use libc::c_char;
use zenoh_ffi::{
    z_closure_reply_move, z_closure_sample, z_closure_sample_move, z_config_default, z_config_move, z_fifo_channel_reply_new, z_fifo_handler_reply_drop, z_fifo_handler_reply_loan, z_fifo_handler_reply_move, z_fifo_handler_reply_recv, z_keyexpr_as_view_string, z_liveliness_declare_subscriber, z_liveliness_declare_token, z_liveliness_get, z_liveliness_token_drop, z_liveliness_token_move, z_liveliness_undeclare_token, z_loaned_sample_t, z_open, z_owned_closure_reply_t, z_owned_closure_sample_t, z_owned_config_t, z_owned_fifo_handler_reply_t, z_owned_liveliness_token_t, z_owned_reply_t, z_owned_session_t, z_owned_subscriber_t, z_reply_drop, z_reply_is_ok, z_reply_loan, z_reply_move, z_reply_ok, z_sample_keyexpr, z_sample_kind, z_sample_kind_t, z_session_drop, z_session_loan, z_session_move, z_sleep_s, z_string_data, z_string_len, z_subscriber_drop, z_subscriber_move, z_view_keyexpr_from_str, z_view_keyexpr_loan, z_view_keyexpr_t, z_view_string_loan, z_view_string_t, Z_CHANNEL_DISCONNECTED, Z_OK
};

#[repr(C)]
struct Context {
    token1_put: bool,
    token2_put: bool,
    token1_drop: bool,
    token2_drop: bool,
}

const TOKEN1_EXPR_STR: [u8; 24] = *b"zenoh/liveliness/test/1\0";
const TOKEN1_EXPR: *const c_char = TOKEN1_EXPR_STR.as_ptr() as *const c_char;
const TOKEN2_EXPR_STR: [u8; 24] = *b"zenoh/liveliness/test/2\0";
const TOKEN2_EXPR: *const c_char = TOKEN2_EXPR_STR.as_ptr() as *const c_char;

extern "C" fn on_receive(s: &mut z_loaned_sample_t, context: *mut c_void) {
    unsafe {
        // context_t* c = (context_t*)context;
        let c = &mut *(context as *mut Context);
        // const z_loaned_keyexpr_t* k = z_sample_keyexpr(s);
        let k = z_sample_keyexpr(s);
        // z_view_string_t ks;
        let mut ks = MaybeUninit::<z_view_string_t>::uninit();
        // z_keyexpr_as_view_string(k, &ks);
        z_keyexpr_as_view_string(k, &mut ks);
        let ks = ks.assume_init();

        // if (z_sample_kind(s) == Z_SAMPLE_KIND_PUT) {
        if z_sample_kind(s) == z_sample_kind_t::PUT {
            //     if (strncmp(token1_expr, z_string_data(z_loan(ks)), z_string_len(z_loan(ks))) == 0) {
            //         c->token1_put = true;
            if *std::slice::from_raw_parts(
                z_string_data(z_view_string_loan(&ks)) as *const u8,
                z_string_len(z_view_string_loan(&ks)),
            ) == TOKEN1_EXPR_STR[..TOKEN1_EXPR_STR.len() - 1]
            {
                c.token1_put = true;
            //     } else if (strncmp(token2_expr, z_string_data(z_loan(ks)), z_string_len(z_loan(ks))) == 0) {
            //         c->token2_put = true;
            //     }
            } else if *std::slice::from_raw_parts(
                z_string_data(z_view_string_loan(&ks)) as *const u8,
                z_string_len(z_view_string_loan(&ks)),
            ) == TOKEN2_EXPR_STR[..TOKEN2_EXPR_STR.len() - 1]
            {
                c.token2_put = true;
            }
        // } else if (z_sample_kind(s) == Z_SAMPLE_KIND_DELETE) {
        } else if z_sample_kind(s) == z_sample_kind_t::DELETE {
            //     if (strncmp(token1_expr, z_string_data(z_loan(ks)), z_string_len(z_loan(ks))) == 0) {
            //         c->token1_drop = true;
            if *std::slice::from_raw_parts(
                z_string_data(z_view_string_loan(&ks)) as *const u8,
                z_string_len(z_view_string_loan(&ks)),
            ) == TOKEN1_EXPR_STR[..TOKEN1_EXPR_STR.len() - 1]
            {
                c.token1_drop = true;
            //     } else if (strncmp(token2_expr, z_string_data(z_loan(ks)), z_string_len(z_loan(ks))) == 0) {
            //         c->token2_drop = true;
            //     }
            } else if *std::slice::from_raw_parts(
                z_string_data(z_view_string_loan(&ks)) as *const u8,
                z_string_len(z_view_string_loan(&ks)),
            ) == TOKEN2_EXPR_STR[..TOKEN2_EXPR_STR.len() - 1]
            {
                c.token2_drop = true;
            }
        }
    }
}

#[test]
fn liveliness_sub() {
    unsafe {
        // const char* expr = "zenoh/liveliness/test/*";
        const EXPR_STR: [u8; 24] = *b"zenoh/liveliness/test/*\0";
        const EXPR: *const c_char = EXPR_STR.as_ptr() as *const c_char;

        // z_owned_session_t s1, s2;
        let mut s1 = MaybeUninit::<z_owned_session_t>::uninit();
        let mut s2 = MaybeUninit::<z_owned_session_t>::uninit();
        // z_owned_config_t c1, c2;
        let mut c1 = MaybeUninit::<z_owned_config_t>::uninit();
        let mut c2 = MaybeUninit::<z_owned_config_t>::uninit();
        // z_config_default(&c1);
        // z_config_default(&c2);
        z_config_default(&mut c1);
        let mut c1 = c1.assume_init();
        z_config_default(&mut c2);
        let mut c2 = c2.assume_init();
        // z_view_keyexpr_t k, k1, k2;
        let mut k = MaybeUninit::<z_view_keyexpr_t>::uninit();
        let mut k1 = MaybeUninit::<z_view_keyexpr_t>::uninit();
        let mut k2 = MaybeUninit::<z_view_keyexpr_t>::uninit();
        // z_view_keyexpr_from_str(&k, expr);
        z_view_keyexpr_from_str(&mut k, EXPR);
        let k = k.assume_init();
        // z_view_keyexpr_from_str(&k1, token1_expr);
        z_view_keyexpr_from_str(&mut k1, TOKEN1_EXPR);
        let k1 = k1.assume_init();
        // z_view_keyexpr_from_str(&k2, token2_expr);
        z_view_keyexpr_from_str(&mut k2, TOKEN2_EXPR);
        let k2 = k2.assume_init();

        // z_open(&s1, z_move(c1), NULL);
        z_open(&mut s1, z_config_move(&mut c1), None);
        let mut s1 = s1.assume_init();
        // z_open(&s2, z_move(c2), NULL);
        z_open(&mut s2, z_config_move(&mut c2), None);
        let mut s2 = s2.assume_init();

        // z_owned_closure_sample_t closure;
        let mut closure = MaybeUninit::<z_owned_closure_sample_t>::uninit();
        // context_t context = {false, false, false, false};
        let mut context = Context {
            token1_put: false,
            token2_put: false,
            token1_drop: false,
            token2_drop: false,
        };
        // z_closure(&closure, on_receive, NULL, (void*)(&context));
        z_closure_sample(
            &mut closure,
            Some(on_receive),
            None,
            &mut context as *mut Context as *mut c_void,
        );
        let mut closure = closure.assume_init();

        // z_owned_subscriber_t sub;
        let mut sub = MaybeUninit::<z_owned_subscriber_t>::uninit();
        // z_liveliness_declare_subscriber(z_loan(s2), &sub, z_loan(k), z_move(closure), NULL);
        z_liveliness_declare_subscriber(
            z_session_loan(&s2),
            &mut sub,
            z_view_keyexpr_loan(&k),
            z_closure_sample_move(&mut closure),
            None,
        );
        let mut sub = sub.assume_init();

        // z_sleep_s(1);
        sleep(Duration::from_secs(1));
        // z_owned_liveliness_token_t t1, t2;
        let mut t1 = MaybeUninit::<z_owned_liveliness_token_t>::uninit();
        let mut t2 = MaybeUninit::<z_owned_liveliness_token_t>::uninit();
        // z_liveliness_declare_token(z_loan(s1), &t1, z_loan(k1), NULL);
        z_liveliness_declare_token(z_session_loan(&s1), &mut t1, z_view_keyexpr_loan(&k1), None);
        let mut t1 = t1.assume_init();
        // z_liveliness_declare_token(z_loan(s1), &t2, z_loan(k2), NULL);
        z_liveliness_declare_token(z_session_loan(&s1), &mut t2, z_view_keyexpr_loan(&k2), None);
        let mut t2 = t2.assume_init();

        // z_sleep_s(1);
        z_sleep_s(1);

        // assert(context.token1_put);
        assert!(context.token1_put);
        // assert(context.token2_put);
        assert!(context.token2_put);

        // z_liveliness_undeclare_token(z_move(t1));
        z_liveliness_undeclare_token(z_liveliness_token_move(&mut t1));
        // z_sleep_s(1);
        sleep(Duration::from_secs(1));

        // assert(context.token1_drop);
        assert!(context.token1_drop);
        // assert(!context.token2_drop);
        assert!(!context.token2_drop);

        // z_liveliness_undeclare_token(z_move(t2));
        z_liveliness_undeclare_token(z_liveliness_token_move(&mut t2));
        // z_sleep_s(1);
        z_sleep_s(1);
        // assert(context.token2_drop);
        assert!(context.token2_drop);

        // z_drop(z_move(sub));
        z_subscriber_drop(z_subscriber_move(&mut sub));
        // z_drop(z_move(s1));
        z_session_drop(z_session_move(&mut s1));
        // z_drop(z_move(s2));
        z_session_drop(z_session_move(&mut s2));
    }
}

#[test]
fn liveliness_get() {
    unsafe {
        // const char* expr = "zenoh/liveliness/test/*";
        const EXPR_STR: [u8; 24] = *b"zenoh/liveliness/test/*\0";
        const EXPR: *const c_char = EXPR_STR.as_ptr() as *const c_char;

        // z_owned_session_t s1, s2;
        let mut s1 = MaybeUninit::<z_owned_session_t>::uninit();
        let mut s2 = MaybeUninit::<z_owned_session_t>::uninit();
        // z_owned_config_t c1, c2;
        let mut c1 = MaybeUninit::<z_owned_config_t>::uninit();
        let mut c2 = MaybeUninit::<z_owned_config_t>::uninit();
        // z_config_default(&c1);
        z_config_default(&mut c1);
        let mut c1 = c1.assume_init();
        // z_config_default(&c2);
        z_config_default(&mut c2);
        let mut c2 = c2.assume_init();
        // z_view_keyexpr_t k, k1;
        let mut k = MaybeUninit::<z_view_keyexpr_t>::uninit();
        let mut k1 = MaybeUninit::<z_view_keyexpr_t>::uninit();
        // z_view_keyexpr_from_str(&k, expr);
        z_view_keyexpr_from_str(&mut k, EXPR);
        let k = k.assume_init();
        // z_view_keyexpr_from_str(&k1, token1_expr);
        z_view_keyexpr_from_str(&mut k1, TOKEN1_EXPR);
        let k1 = k1.assume_init();

        // z_open(&s1, z_move(c1), NULL);
        z_open(&mut s1, z_config_move(&mut c1), None);
        let mut s1 = s1.assume_init();

        // z_open(&s2, z_move(c2), NULL);
        z_open(&mut s2, z_config_move(&mut c2), None);
        let mut s2 = s2.assume_init();

        // z_sleep_s(1);
        z_sleep_s(1);

        // z_owned_liveliness_token_t t1;
        let mut t1 = MaybeUninit::<z_owned_liveliness_token_t>::uninit();
        // z_liveliness_declare_token(z_loan(s1), &t1, z_loan(k1), NULL);
        z_liveliness_declare_token(z_session_loan(&s1), &mut t1, z_view_keyexpr_loan(&k1), None);
        let mut t1 = t1.assume_init();
        // z_sleep_s(1);
        z_sleep_s(1);

        // z_owned_fifo_handler_reply_t handler;
        let mut handler = MaybeUninit::<z_owned_fifo_handler_reply_t>::uninit();
        // z_owned_closure_reply_t cb;
        let mut cb = MaybeUninit::<z_owned_closure_reply_t>::uninit();
        // z_fifo_channel_reply_new(&cb, &handler, 3);
        z_fifo_channel_reply_new(&mut cb, &mut handler, 3);
        let mut cb = cb.assume_init();
        let mut handler = handler.assume_init();

        // z_liveliness_get(z_loan(s2), z_loan(k), z_move(cb), NULL);
        z_liveliness_get(
            z_session_loan(&s2),
            z_view_keyexpr_loan(&k),
            z_closure_reply_move(&mut cb),
            None,
        );
        // z_owned_reply_t reply;
        let mut reply = MaybeUninit::<z_owned_reply_t>::uninit();
        // assert(z_recv(z_loan(handler), &reply) == Z_OK);
        assert!(z_fifo_handler_reply_recv(z_fifo_handler_reply_loan(&handler), &mut reply) == Z_OK);
        let mut reply = reply.assume_init();
        // assert(z_reply_is_ok(z_loan(reply)));
        assert!(z_reply_is_ok(z_reply_loan(&reply)));
        // const z_loaned_keyexpr_t* reply_keyexpr = z_sample_keyexpr(z_reply_ok(z_loan(reply)));
        let reply_ok = z_reply_ok(z_reply_loan(&reply));
        assert!(!reply_ok.is_null());
        let reply_keyexpr = z_sample_keyexpr(reply_ok.as_ref().unwrap());
        // z_view_string_t reply_keyexpr_s;
        let mut reply_keyexpr_s = MaybeUninit::<z_view_string_t>::uninit();
        // z_keyexpr_as_view_string(reply_keyexpr, &reply_keyexpr_s);
        z_keyexpr_as_view_string(reply_keyexpr, &mut reply_keyexpr_s);
        let reply_keyexpr_s = reply_keyexpr_s.assume_init();
        // assert(strncmp(token1_expr, z_string_data(z_loan(reply_keyexpr_s)), z_string_len(z_loan(reply_keyexpr_s))) == 0);
        assert!(
            *std::slice::from_raw_parts(
                z_string_data(z_view_string_loan(&reply_keyexpr_s)) as *const u8,
                z_string_len(z_view_string_loan(&reply_keyexpr_s))
            ) == TOKEN1_EXPR_STR[..TOKEN1_EXPR_STR.len() - 1]
        );

        // z_drop(z_move(reply));
        z_reply_drop(z_reply_move(&mut reply));
        // assert(z_recv(z_loan(handler), &reply) == Z_CHANNEL_DISCONNECTED);
        let mut reply = MaybeUninit::<z_owned_reply_t>::uninit();
        assert!(
            z_fifo_handler_reply_recv(z_fifo_handler_reply_loan(&handler), &mut reply)
                == Z_CHANNEL_DISCONNECTED
        );

        // z_drop(z_move(t1));
        z_liveliness_token_drop(z_liveliness_token_move(&mut t1));
        // z_drop(z_move(handler));
        z_fifo_handler_reply_drop(z_fifo_handler_reply_move(&mut handler));
        // z_sleep_s(1);
        z_sleep_s(1);
        // z_fifo_channel_reply_new(&cb, &handler, 3);
        let mut handler = MaybeUninit::<z_owned_fifo_handler_reply_t>::uninit();
        let mut cb = MaybeUninit::<z_owned_closure_reply_t>::uninit();
        z_fifo_channel_reply_new(&mut cb, &mut handler, 3);
        let mut cb = cb.assume_init();
        let mut handler = handler.assume_init();

        // z_liveliness_get(z_loan(s2), z_loan(k), z_move(cb), NULL);
        z_liveliness_get(
            z_session_loan(&s2),
            z_view_keyexpr_loan(&k),
            z_closure_reply_move(&mut cb),
            None,
        );
        // assert(z_recv(z_loan(handler), &reply) == Z_CHANNEL_DISCONNECTED);
        assert!(
            z_fifo_handler_reply_recv(z_fifo_handler_reply_loan(&handler), &mut reply)
                == Z_CHANNEL_DISCONNECTED
        );

        // z_drop(z_move(handler));
        z_fifo_handler_reply_drop(z_fifo_handler_reply_move(&mut handler));
        // z_drop(z_move(s1));
        z_session_drop(z_session_move(&mut s1));
        // z_drop(z_move(s2));
        z_session_drop(z_session_move(&mut s2));
    }
}

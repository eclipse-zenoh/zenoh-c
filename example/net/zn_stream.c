/*
 * Copyright (c) 2017, 2020 ADLINK Technology Inc.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
 * which is available at https://www.apache.org/licenses/LICENSE-2.0.
 *
 * SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
 *
 * Contributors:
 *   ADLINK zenoh team, <zenoh@adlink-labs.tech>
 */
#include <stdio.h>
#include <unistd.h>
#include "zenoh.h"

int main(int argc, char **argv) {
  char *uri = "/demo/example/zenoh-c-stream";
  if (argc > 1) {
    uri = argv[1];
  }
  char *value = "Stream from C!";
  if (argc > 2) {
    value = argv[2];
  }
  char *locator = 0;
  if (argc > 3) {
    locator = argv[3];
  }

  printf("Openning session...\n");
  zn_session_p_result_t r_z = zn_open(locator, 0, 0);
  ASSERT_RESULT(r_z, "Unable to open session.\n")
  zn_session_t *z = r_z.value.session;
  zn_start_recv_loop(z);  
  
  printf("Declaring Publisher on '%s'...\n", uri);
  zn_pub_p_result_t r = zn_declare_publisher(z, uri);
  ASSERT_P_RESULT(r, "Unable to declare publisher.\n");  
  zn_pub_t *pub = r.value.pub;

  char buf[256];
  for(int idx = 0; 1; ++idx) {
    sleep(1);
    sprintf(buf, "[%4d] %s", idx, value);
    printf("Streaming Data ('%s': '%s')...\n", uri, buf);
    zn_stream_data(pub, (const unsigned char *)buf, strlen(buf));
  }

  zn_undeclare_publisher(pub);
  zn_close(z);
  zn_stop_recv_loop(z);
  return 0;
}

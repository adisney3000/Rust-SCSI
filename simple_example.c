#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <inttypes.h>
#include <string.h>
#include "scsi.h"

int main(int argc, char **argv)
{
  if(argc < 2)
  {
    fprintf(stderr, "Usage: example_c <device_path>\n");
    return EXIT_SUCCESS;
  }

  Device *device = device_new();
  if(!device_open(device, argv[1]))
    return EXIT_FAILURE;

  Rewind rewind_cmd = {
      .immed = false
  };
  DeviceStatus *status = device_issue_rewind(device, &rewind_cmd);
  if(status == NULL)
    return EXIT_FAILURE;
  device_status_to_stdout(status);
  device_status_free(status);

  Read6 read_cmd = {0};
  read_cmd.transfer_length = 80;
  uint8_t *buf = calloc(80, 1);
  status = device_issue_read6(device, &read_cmd, buf, 80);
  if(status == NULL)
    return EXIT_FAILURE;

  device_status_to_stdout(status);
  device_status_free(status);
  for(int i = 0; i < 80; i++)
    printf("%3"PRIx8, buf[i]);
  printf("\n");

  memset(buf, 0, 80);
  status = device_issue_read6(device, &read_cmd, buf, 80);
  if(status == NULL)
    return EXIT_FAILURE;

  device_status_to_stdout(status);
  read_6_status_to_stdout(&read_cmd, status);
  device_status_free(status);
  for(int i = 0; i < 80; i++)
    printf("%3"PRIx8, buf[i]);
  printf("\n");

  device_delete(device);
  free(buf);
}

#include "reveldb/c.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

const char* phase = "";
static char dbname[200];

static void StartPhase(const char* name) {
  fprintf(stderr, "=== Test %s\n", name);
  phase = name;
}

static const char* GetTempDir(void) {
    const char* ret = getenv("TEST_TMPDIR");
    if (ret == NULL || ret[0] == '\0')
        ret = "/tmp";
    return ret;
}

#define CheckCondition(cond)                                            \
  if (!(cond)) {                                                        \
    fprintf(stderr, "%s:%d: %s: %s\n", __FILE__, __LINE__, phase, #cond); \
    abort();                                                            \
  }

static void CmpDestroy(void* arg) { }

static int CmpCompare(void* arg, const char* a, size_t alen,
                      const char* b, size_t blen) {
  int n = (alen < blen) ? alen : blen;
  int r = memcmp(a, b, n);
  if (r == 0) {
    if (alen < blen) r = -1;
    else if (alen > blen) r = +1;
  }
  return r;
}

static const char* CmpName(void* arg) {
  return "foo";
}

int main(int argc, char** argv) {
  leveldb_t* db;
  leveldb_comparator_t* cmp;
  leveldb_cache_t* cache;
  leveldb_env_t* env;
  leveldb_options_t* options;
  leveldb_readoptions_t* roptions;
  leveldb_writeoptions_t* woptions;
  char* err = NULL;
  int run = -1;

  CheckCondition(leveldb_major_version() >= 1);
  CheckCondition(leveldb_minor_version() >= 1);

  snprintf(dbname, sizeof(dbname),
           "%s/leveldb_c_test-%d",
           GetTempDir(),
           ((int) geteuid()));

  StartPhase("create_objects");
  cmp = leveldb_comparator_create(NULL, CmpDestroy, CmpCompare, CmpName);
  env = leveldb_create_default_env();
  cache = leveldb_cache_create_lru(100000);

  options = leveldb_options_create();
  leveldb_options_set_comparator(options, cmp);
  leveldb_options_set_error_if_exists(options, 1);
  leveldb_options_set_cache(options, cache);
  /*
  leveldb_options_set_env(options, env);
  leveldb_options_set_info_log(options, NULL);
  leveldb_options_set_write_buffer_size(options, 100000);
  leveldb_options_set_paranoid_checks(options, 1);
  leveldb_options_set_max_open_files(options, 10);
  leveldb_options_set_block_size(options, 1024);
  leveldb_options_set_block_restart_interval(options, 8);
  leveldb_options_set_compression(options, leveldb_no_compression);

  roptions = leveldb_readoptions_create();
  leveldb_readoptions_set_verify_checksums(roptions, 1);
  leveldb_readoptions_set_fill_cache(roptions, 0);

  woptions = leveldb_writeoptions_create();
  leveldb_writeoptions_set_sync(woptions, 1);
  */

  StartPhase("cleanup");
  leveldb_close(db);
  leveldb_options_destroy(options);
  leveldb_cache_destroy(cache);
  //leveldb_comparator_destroy(cmp);

  fprintf(stderr, "PASS\n");
  return 0;
}

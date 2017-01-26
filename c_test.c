#include "reveldb/c.h"

#include <stdio.h>
#include <stdlib.h>
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
  //cmp = leveldb_comparator_create(NULL, CmpDestroy, CmpCompare, CmpName);
  env = leveldb_create_default_env();
  cache = leveldb_cache_create_lru(100000);

  fprintf(stderr, "PASS\n");
  return 0;
}

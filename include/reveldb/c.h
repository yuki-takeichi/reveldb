#include <stddef.h>

extern int leveldb_major_version();
extern int leveldb_minor_version();

typedef struct leveldb_t               leveldb_t;
typedef struct leveldb_cache_t         leveldb_cache_t;
typedef struct leveldb_comparator_t    leveldb_comparator_t;
typedef struct leveldb_env_t           leveldb_env_t;
typedef struct leveldb_filelock_t      leveldb_filelock_t;
typedef struct leveldb_filterpolicy_t  leveldb_filterpolicy_t;
typedef struct leveldb_iterator_t      leveldb_iterator_t;
typedef struct leveldb_logger_t        leveldb_logger_t;
typedef struct leveldb_options_t       leveldb_options_t;
typedef struct leveldb_randomfile_t    leveldb_randomfile_t;
typedef struct leveldb_readoptions_t   leveldb_readoptions_t;
typedef struct leveldb_seqfile_t       leveldb_seqfile_t;
typedef struct leveldb_snapshot_t      leveldb_snapshot_t;
typedef struct leveldb_writablefile_t  leveldb_writablefile_t;
typedef struct leveldb_writebatch_t    leveldb_writebatch_t;
typedef struct leveldb_writeoptions_t  leveldb_writeoptions_t;

extern leveldb_t* leveldb_open(
    //const leveldb_options_t* options,
    //const char* name,
    //char** errptr);
    );

extern void leveldb_close(leveldb_t* db);

extern void leveldb_free(void* ptr);

/* Cache */

extern leveldb_cache_t* leveldb_cache_create_lru(size_t capacity);
extern void leveldb_cache_destroy(leveldb_cache_t* cache);

/* Env */

extern leveldb_env_t* leveldb_create_default_env();
extern void leveldb_env_destroy(leveldb_env_t*);

/* Comparator */

extern leveldb_comparator_t* leveldb_comparator_create(
    void* state,
    void (*destructor)(void*),
    int (*compare)(
        void*,
        const char* a, size_t alen,
        const char* b, size_t blen),
    const char* (*name)(void*));
extern void leveldb_comparator_destroy(leveldb_comparator_t*);

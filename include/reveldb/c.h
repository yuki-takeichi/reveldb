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

/* Options */

extern leveldb_options_t* leveldb_options_create();
extern void leveldb_options_destroy(leveldb_options_t*);
extern void leveldb_options_set_comparator(
    leveldb_options_t*,
    leveldb_comparator_t*);
extern void leveldb_options_set_filter_policy(
    leveldb_options_t*,
    leveldb_filterpolicy_t*);
extern void leveldb_options_set_create_if_missing(
    leveldb_options_t*, unsigned char);
extern void leveldb_options_set_error_if_exists(
    leveldb_options_t*, unsigned char);
extern void leveldb_options_set_paranoid_checks(
    leveldb_options_t*, unsigned char);
extern void leveldb_options_set_env(leveldb_options_t*, leveldb_env_t*);
extern void leveldb_options_set_info_log(leveldb_options_t*, leveldb_logger_t*);
extern void leveldb_options_set_write_buffer_size(leveldb_options_t*, size_t);
extern void leveldb_options_set_max_open_files(leveldb_options_t*, int);
extern void leveldb_options_set_cache(leveldb_options_t*, leveldb_cache_t*);
extern void leveldb_options_set_block_size(leveldb_options_t*, size_t);
extern void leveldb_options_set_block_restart_interval(leveldb_options_t*, int);

enum {
  leveldb_no_compression = 0,
  leveldb_snappy_compression = 1
};
extern void leveldb_options_set_compression(leveldb_options_t*, int);


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

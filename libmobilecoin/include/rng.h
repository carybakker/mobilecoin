// Copyright (c) 2018-2022 The MobileCoin Foundation

#ifndef RNG_H_
#define RNG_H_

#include "common.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct _ChaCha20Rng ChaCha20Rng;

ChaCha20Rng* MC_NULLABLE mc_chacha20_rng_create_with_long(int64_t value);

uint32_t mc_chacha20_rng_next_int(ChaCha20Rng* MC_NULLABLE chacha20_rng)
MC_ATTRIBUTE_NONNULL(1);

uint64_t mc_chacha20_rng_next_long(ChaCha20Rng* MC_NULLABLE chacha20_rng)
MC_ATTRIBUTE_NONNULL(1);

void mc_chacha20_rng_free(ChaCha20Rng* MC_NULLABLE chacha20_rng)
MC_ATTRIBUTE_NONNULL(1);


#ifdef __cplusplus
}
#endif

#endif /* RNG_H_ */

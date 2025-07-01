#pragma once

#ifdef DOCS
#define ALIGN(n)
#else
#ifdef _MSC_VER
#define ALIGN(n) __declspec(align(n))
#else
#define ALIGN(n) __attribute__((aligned(n)))
#endif
#endif

#ifdef DOCS
#define ZENOHC_API
#elif defined(ZENOHC_DYN_LIB) && defined(_MSC_VER)
#define ZENOHC_API __declspec(dllimport)
#elif defined(_MSC_VER)
#define ZENOHC_API
#else
#define ZENOHC_API __attribute__((visibility("default")))
#endif

#ifndef __UTIL_H__
#define __UTIL_H__

#define SET_BIT(addr, bit) (addr |= (1<<bit))
#define CLEAR_BIT(addr, bit) (addr &= ~(1<<bit))

#define MIN(a, b) (((a)<(b)) ? (a):(b))

#endif
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    long capacity;
    long size;
    long *elements;
} Longlist;

Longlist* emptyLonglist() {
    Longlist *new = malloc(sizeof(Longlist));
    new->elements = malloc(10 * sizeof(long));
    new->capacity = 10;
    new->size = 0;
    return new;
}

void appendToLonglist(Longlist* list, long element) {
    if (list->size > list->capacity) {
        fprintf(stderr, "List size %ld exceeds capacity %ld\n", list->size, list->capacity);
        exit(1);
    } else if (list->size == list->capacity) {
        list->elements = realloc(list->elements, (list->capacity + 10) * sizeof(long));
        list->capacity += 10;
    }
    list->elements[list->size] = element;
    list->size++;
}

typedef struct {
    long source;
    long range;
    long dest;
} Map;

typedef struct {
    long capacity;
    long size;
    Map *elements;
} Maplist;


Maplist* emptyMaplist() {
    Maplist *new = malloc(sizeof(Maplist));
    new->elements = malloc(10 * sizeof(Map));
    new->capacity = 10;
    new->size = 0;
    return new;
}

void appendToMaplist(Maplist* list, Map element) {
    if (list->size > list->capacity) {
        fprintf(stderr, "List size %ld exceeds capacity %ld\n", list->size, list->capacity);
        exit(1);
    } else if (list->size == list->capacity) {
        list->elements = realloc(list->elements, (list->capacity + 10) * sizeof(Map));
        list->capacity += 10;
    }
    list->elements[list->size] = element;
    list->size++;
}

typedef struct {
    long capacity;
    long size;
    Maplist **elements;
} Maps;


Maps* emptyMaps() {
    Maps *new = malloc(sizeof(Maps));
    new->elements = malloc(10 * sizeof(Maplist));
    new->capacity = 10;
    new->size = 0;
    return new;
}

void appendToMaps(Maps* list, Maplist* element) {
    if (list->size > list->capacity) {
        fprintf(stderr, "List size %ld exceeds capacity %ld\n", list->size, list->capacity);
        exit(1);
    } else if (list->size == list->capacity) {
        list->elements = realloc(list->elements, (list->capacity + 10) * sizeof(Maplist));
        list->capacity += 10;
    }
    list->elements[list->size] = element;
    list->size++;
}

long findNextSection(FILE *in_file){
    char ch;
    while (!feof(in_file)){
        ch = fgetc (in_file);
        if(ch == ':'){
            return 1;
        }
    }
    return 0;
}

void readInput(Longlist *seeds, Maps *maps) {
    FILE *in_file  = fopen("input.txt", "r");
    if (in_file == NULL) {
        fprintf(stderr, "Input file not found");
        exit(1);
    }
    long n;
    fscanf(in_file, "seeds:");
    while (fscanf(in_file, " %ld", &n) == 1) 
        appendToLonglist(seeds, n);
    long s,r,d;
    while(findNextSection(in_file)){
        Maplist* xtoy = emptyMaplist();
        while (fscanf(in_file, "%ld %ld %ld\n", &d, &s ,&r) == 3){
            Map m = {s,r,d};
            appendToMaplist(xtoy, m);
        }
        appendToMaps(maps, xtoy);
    }
    fclose(in_file);
}

long getSolutionPart1(Longlist *seeds, Maps *maps) {
    Longlist *locations = emptyLonglist();
    for (long i = 0; i < seeds->size; ++i) {
        long seed = seeds->elements[i];
        for(long j = 0; j < maps->size; j++){
            Maplist *maplist = maps->elements[j];
            for(long k = 0; k < maplist->size; k++){
                Map map = maplist->elements[k];
                if(seed >= map.source && seed <= map.source+map.range){
                    seed = map.dest + seed - map.source;
                    break;
                }
            }
        }
        appendToLonglist(locations, seed);
    }
    long min = locations->elements[0];
    for(long i = 1; i < locations->size; i++)
        min = locations->elements[i] < min ? locations->elements[i] : min;
    return min;
}

long getSolutionPart2(Longlist *seeds, Maps *maps) {
    Longlist *locations = emptyLonglist();
    for (long i = 0; i < seeds->size; i+=2) {
        for(int i_ = 0; i_ < seeds->elements[i+1]; i_++){
            long seed = seeds->elements[i] + i_;
            for(long j = 0; j < maps->size; j++){
                Maplist *maplist = maps->elements[j];
                for(long k = 0; k < maplist->size; k++){
                    Map map = maplist->elements[k];
                    if(seed >= map.source && seed < map.source+map.range){
                        seed = map.dest + seed - map.source;
                        break;
                    }
                }
            }
            appendToLonglist(locations, seed);
        }
    }
    long min = locations->elements[0];
    for(long i = 1; i < locations->size; i++)
        min = locations->elements[i] < min ? locations->elements[i] : min;
    return min;
}

int main() {
    Longlist *seeds = emptyLonglist();
    Maps *maps = emptyMaps();
    readInput(seeds, maps);

    char* part = getenv("part");

    if (part != NULL && strcmp("part2", part) == 0) {
        printf("%ld\n", getSolutionPart2(seeds, maps));
    } else {
        printf("%ld\n", getSolutionPart1(seeds, maps));
    }

   return 0;
}
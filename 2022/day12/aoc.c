#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const int dx[4] = {-1, 0, 1, 0};
const int dy[4] = {0, -1, 0, 1};

typedef struct
{
    int x, y, step;
} Point;
typedef struct
{
    int capacity;
    Point *mem;
    Point *start;
    Point *end;
} PointQueue;

PointQueue *emptyQueue(int initial_capacity)
{
    PointQueue *new = malloc(sizeof(PointQueue));
    Point *mem = malloc(sizeof(Point) * initial_capacity);
    new->capacity = initial_capacity;
    new->mem = mem;
    new->start = mem;
    new->end = mem;
    return new;
}

void increase_capacity(PointQueue *queue)
{
    printf("Increase\n");
    int new_cap = queue->capacity * 2;
    Point *new_mem = malloc(sizeof(Point) * new_cap);
    Point *start = new_mem;
    Point *ptr = queue->start;
    for (; ptr != queue->end; ptr++)
    {
        if (ptr - queue->mem >= sizeof(Point) * queue->capacity)
            ptr = queue->mem;
        else
            *(new_mem++) = *ptr;
    }
    queue->mem = start;
    queue->start = start;
    queue->end = new_mem;
}

char is_not_empty(PointQueue *queue)
{
    return queue->start != queue->end;
}

void push_back(PointQueue *queue, Point p)
{
    *(queue->end++) = p;
    if ((queue->end - queue->mem) > queue->capacity * sizeof(Point))
        queue->end = queue->mem;
    if (queue->start == queue->end)
        increase_capacity(queue);
}

Point pop_front(PointQueue *queue)
{
    if (!is_not_empty(queue))
    {
        fprintf(stderr, "Trying to pop empty queue\n");
        printf("Trying to pop empty queue\n");
        exit(1);
    }
    Point p = *(queue->start++);
    if ((queue->start - queue->mem) > queue->capacity * sizeof(Point))
        queue->start = queue->mem;
    if (queue->start == queue->end) // Empty queue
    {
        queue->start = queue->mem;
        queue->end = queue->mem;
    }
    return p;
}

typedef struct
{
    int capacity;
    int size;
    Point *elements;
} PointList;

PointList *emptyPointList()
{
    PointList *new = malloc(sizeof(PointList));
    new->elements = malloc(1000 * sizeof(Point));
    new->capacity = 1000;
    new->size = 0;
    return new;
}

void appendToPointList(PointList *list, Point element)
{
    if (list->size > list->capacity)
    {
        fprintf(stderr, "List size %d exceeds capacity %d\n", list->size, list->capacity);
        exit(1);
    }
    else if (list->size == list->capacity)
    {
        list->elements = realloc(list->elements, (list->capacity * 2) * sizeof(int));
        list->capacity *= 2;
    }
    list->elements[list->size] = element;
    list->size++;
}

char existInPointList(PointList *list, Point element)
{
    for (int i = 0; i < list->size; i++)
    {
        Point p = list->elements[i];
        if (element.x == p.x && element.y == p.y)
        {
            return 1;
        }
    }
    return 0;
}

char *readInput(int *row_count, int *col_count, int *start_pos, int *end_pos)
{
    FILE *in_file = fopen("input.txt", "r"); // read only
    if (in_file == NULL)
    {
        fprintf(stderr, "Input file not found");
        printf("Input file not found");
        printf("Input file not found");
        exit(1);
    }
    char c;
    char first_row = 1;
    int rows = 0, cols = 0;
    for (c = getc(in_file); c != EOF; c = getc(in_file))
    {
        if (c == '\n')
        {
            rows++;
            first_row = 0;
        }
        else if (first_row)
            cols++;
    }
    char *chars = malloc(sizeof(char) * rows * cols);
    fclose(in_file);
    in_file = fopen("input.txt", "r");
    int pos = 0;
    for (c = getc(in_file); c != EOF; c = getc(in_file))
    {
        if (c == '\n')
            continue;
        if (c == 'S')
            *start_pos = pos;
        if (c == 'E')
            *end_pos = pos;
        chars[pos++] = c;
    }
    fclose(in_file);
    *row_count = rows;
    *col_count = cols;
    return chars;
}

int point_to_pos(Point p, int cols)
{
    return p.y * cols + p.x;
}

int getSolutionPart1(char *map, int rows, int cols, int start_pos)
{
    PointList *visited = emptyPointList();
    PointQueue *queue = emptyQueue(rows * cols / 2);
    push_back(queue, (Point){.x = start_pos % cols, .y = start_pos / cols, .step = 0});
    while (is_not_empty(queue))
    {
        Point p = pop_front(queue);
        if (map[point_to_pos(p, cols)] == 'E')
        {
            return p.step;
        }
        for (int i = 0; i < 4; i++)
        {
            int pdx = p.x + dx[i], pdy = p.y + dy[i];
            if (pdx < 0 || pdx > cols || pdy < 0 || pdy > rows)
                continue;
            Point next = (Point){.x = pdx, .y = pdy, .step = p.step + 1};
            if (existInPointList(visited, next))
                continue;
            int curr_height = map[point_to_pos(p, cols)];
            curr_height = curr_height == 'S' ? 'a' : curr_height;
            int to_height = map[point_to_pos(next, cols)];
            to_height = to_height == 'E' ? 'z' : to_height;
            if (curr_height + 1 >= to_height)
            {
                appendToPointList(visited, next);
                push_back(queue, next);
            }
        }
    }
    return -1;
}

int getSolutionPart2(char *map, int rows, int cols, int start_pos)
{
    PointList *visited = emptyPointList();
    PointQueue *queue = emptyQueue(rows * cols / 2);
    push_back(queue, (Point){.x = start_pos % cols, .y = start_pos / cols, .step = 0});
    while (is_not_empty(queue))
    {
        Point p = pop_front(queue);
        char c = map[point_to_pos(p, cols)];
        c = c == 'S' ? 'a' : c;
        if (map[point_to_pos(p, cols)] == 'a')
        {
            return p.step;
        }

        for (int i = 0; i < 4; i++)
        {
            int pdx = p.x + dx[i], pdy = p.y + dy[i];
            if (pdx < 0 || pdx > cols || pdy < 0 || pdy > rows)
                continue;
            Point next = (Point){.x = pdx, .y = pdy, .step = p.step + 1};
            if (existInPointList(visited, next))
                continue;
            char curr_height = map[point_to_pos(p, cols)];
            curr_height = curr_height == 'E' ? 'z' : curr_height;
            char to_height = map[point_to_pos(next, cols)];
            to_height = to_height == 'S' ? 'a' : to_height;
            if (curr_height - 1 <= to_height)
            {
                appendToPointList(visited, next);
                push_back(queue, next);
            }
        }
    }
    return -1;
}

int main()
{
    int rows, cols, start_pos, end_pos;
    char *data = readInput(&rows, &cols, &start_pos, &end_pos);
    printf("Start pos: %d, rows: %d, cols: %d\n", start_pos, rows, cols);
    char *part = getenv("part");

    if (part != NULL && strcmp("part2", part) == 0)
    {
        printf("%d\n", getSolutionPart2(data, rows, cols, end_pos));
    }
    else
    {
        printf("%d\n", getSolutionPart1(data, rows, cols, start_pos));
    }
    free(data);

    return 0;
}
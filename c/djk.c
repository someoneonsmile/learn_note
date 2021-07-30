#include <stdio.h>
#include <stdlib.h>
#include <time.h>

// 点的数量
#define N 10
// 最大距离
#define MAX_DISTANCE 100


// 罗马数字
char ROME_NUM[N][5] = { {"I"}, {"II"}, {"III"}, {"IV"}, {"V"}, {"VI"}, {"VII"}, {"VIII"}, {"IX"}, {"X"}};

// 输入矩阵
int nums[N][N];

// 输入大小
int n;


/*
 * 生成 [0, n) 随机数
 */
int rand_n(int n) {
    return rand() % n;
}


/*
 * 输入数组
 */
void gen_arr() {
    int x;
    for ( int i = 0; i < n; ++i) {
        for (int j = i; j < n; ++j) {
            if (i == j) {
                // 自身的距离为 0
                nums[i][j] = 0;
            } else {
                // 输入距离
                printf("enter %d-%d distance: ", i + 1, j + 1);
                scanf("%d", &x);
                printf("\n");
                // 输入超过最大距离, 距离改为最大距离
                if (x > MAX_DISTANCE) {
                    x = MAX_DISTANCE;
                }
                nums[i][j] = x;
                // 距离双向相等
                nums[j][i] = nums[i][j];
            }
        }
    }
}


/*
 * 打印矩阵
 */
void print_two(int nums[N][N]) {
    // 打印列号
    printf("*");
    for (int i = 0; i < n; ++i) {
        printf("\t%s", ROME_NUM[i]);
    }
    printf("\n");

    // 打印矩阵
    for (int i = 0; i < n; ++i) {
        printf("%s", ROME_NUM[i]);
        for (int j = 0; j < n; ++j) {
            printf("\t%d", nums[i][j]);
        }
        printf("\n");
    }
}


/*
 * 打印点 p 到其它点的最短距离
 */
void djk(int nums[N][N], int p) {
    int d[N];
    int visit[N];
    int prev[N];

    // 初始化
    for (int i = 0; i < n; ++i) {
        d[i] = nums[p][i];
        visit[i] = 0;
        prev[i] = p;
    }

    /*
     * 加入起点的为集合 P, 未加入的为集合 Q
     */

    // p 加入集合 P
    d[p] = 0;
    visit[p] = 1;

    // 集合 P 大小, p_size == n 时遍历完成
    int p_size = 1;

    while (p_size < n) {
        // 从集合 Q 中找出距离 p 最近的点加入集合 P
        int min_d = MAX_DISTANCE + 1;
        int min_index = 0;
        for (int i = 0; i < n; ++i) {
            if (visit[i] == 0 && d[i] < min_d) {
                min_d = d[i];
                min_index = i;
            }
        }
        visit[min_index] = 1;
        ++p_size;

        // 通过 min_index 更新集合 Q 到 p 的最短距离
        for (int i = 0; i < n; ++i) {
            if (visit[i] == 0 && nums[min_index][i] + min_d < d[i]) {
                d[i] = min_d + nums[min_index][i];
                prev[i] = min_index;
            }
        }
    }

    // 打印最短路径跟距离
    for (int i = 0; i < n; ++i) {
        if (i != p) {
            printf("%d - %d distance=%d\t", p + 1, i + 1, d[i]);

            // 打印路径
            int path[N];
            int index = 0;

            int j = i;
            while (j != p) {
                path[index++] = j;
                j = prev[j];
            }

            printf("path: %d", p + 1);
            while (--index >= 0) {
                printf(" -> %d", path[index] + 1);
            }
            printf("\n");
        }
    }
}


int main() {

    printf("enter the node num (less than 10): ");
    scanf("%d", &n);
    printf("\n");
    if (n > N) {
        n = N;
    }

    // 生成数组
    gen_arr();
    printf("the arr: \n");
    print_two(nums);

    // 输入节点
    int p;
    printf("enter the start node: ");
    scanf("%d", &p);
    printf("\n");
    if (p > n) {
        p = n;
    }
    // 节点索引
    p--;

    // 打印 p 到其它点的最短路径
    djk(nums, p);

    return 0;
}





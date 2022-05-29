#include <bits/stdc++.h>
using namespace std;

const int SIZE = 8;
typedef unsigned long long ull;

void debug_board(vector<vector<int>> vec)
{
    for (int i = (int)(vec.size()) - 1; i >= 0; i--)
    {
        printf("%d ", i + 1);
        for (int j = 0; j < (int)vec[i].size(); j++)
        {
            printf("%c", vec[i][j] ? 'X' : '.');
        }
        printf("\n");
    }
    printf("\n  ");
    for (int i = 0; i < (int)vec[0].size(); i++)
        printf("%c", 'A' + i);
    printf("\n");
}

void debug_relevance(vector<pair<int, int>> vec)
{
    vector<vector<int>> board(SIZE, vector<int>(SIZE, 0));
    for (auto v : vec)
    {
        board[v.first][v.second] = 1;
    }
    debug_board(board);
}

void debug_mask(ull msk)
{
    cout << "MASK: " << msk << endl;
    vector<vector<int>> board(SIZE, vector<int>(SIZE, 0));
    for (int i = 0; i < SIZE * SIZE; i++)
    {
        if ((1ULL << i) & msk)
        {
            board[i / SIZE][i % SIZE] = 1;
        }
    }
    debug_board(board);
}

void debug_bits(ull msk)
{
    for (int i = 63; i >= 0; i--)
    {
        if ((1ULL << i) & msk)
            printf("1");
        else
            printf("0");
    }
    printf("\n");
}

int get_idx(pair<int, int> p)
{
    return p.first * SIZE + p.second;
}

vector<pair<ull, ull>> get_table(int x, int y)
{
    vector<pair<int, int>> rel;
    for (int i = 0; i < SIZE; i++)
    {
        if (i >= 1 && i <= 6 && (i != y))
            rel.push_back({x, i});
    }
    for (int j = 0; j < SIZE; j++)
    {
        if (j >= 1 && j <= 6 && (j != x))
            rel.push_back({j, y});
    }
    ull msk;
    vector<pair<ull, ull>> vec;
    int board[SIZE][SIZE];
    ull atk = 0;
    for (int i = 0; i < (1 << ((int)rel.size())); i++)
    {
        msk = atk = 0;
        memset(board, 0, sizeof(board));
        for (int j = 0; j < (int)rel.size(); j++)
        {
            if (i & (1 << j))
            {
                msk |= (1ULL << get_idx(rel[j]));
                board[rel[j].first][rel[j].second] = 1;
            }
        }
        for (int j = y; j >= 0; j--)
        {
            atk |= (1ULL << get_idx({x, j}));
            if (board[x][j])
                break;
        }
        for (int j = y; j < SIZE; j++)
        {
            atk |= (1ULL << get_idx({x, j}));
            if (board[x][j])
                break;
        }
        for (int j = x; j >= 0; j--)
        {
            atk |= (1ULL << get_idx({j, y}));
            if (board[j][y])
                break;
        }
        for (int j = x; j < SIZE; j++)
        {
            atk |= (1ULL << get_idx({j, y}));
            if (board[j][y])
                break;
        }
        vec.push_back({msk, atk});
    }
    return vec;
}

ull get_hash(ull msk, ull mul, ull sh)
{
    msk = (msk >> sh);
    return msk * mul;
}

const int TABLE_SIZE = (1 << 16);
ull table[TABLE_SIZE];
int vis[TABLE_SIZE];

pair<int, int> get_magic(int x, int y)
{
    const int shift = 10;
    int cnt = 0;
    memset(vis, -1, sizeof(vis));
    auto pairs = get_table(x, y);
    ull mul = 0;
    std::function<int(int, int)> rec;
    rec = [&](int n, int k) -> int
    {
        if (n == 64)
        {
            if (!k)
                return 0;
            cnt++;
            for (auto v : pairs)
            {
                int hsh = get_hash(v.first, mul, shift);
                if (hsh >= TABLE_SIZE)
                    hsh &= (TABLE_SIZE - 1);
                if (vis[hsh] != cnt)
                {
                    vis[hsh] = cnt;
                    table[hsh] = v.second;
                }
                if (table[hsh] != v.second)
                    return 0;
            }
            return 1;
        }
        else
        {
            mul &= (~(1ULL << n));
            if (rec(n + 1, k))
                return 1;
            if (k > 0)
            {
                mul |= (1ULL << n);
                if (rec(n + 1, k - 1))
                    return 1;
            }
            return 0;
        }
    };
    for (int i = 1; i <= 10; i++)
    {
        printf("Checking: %d ones\n", i);
        if (rec(0, i))
        {
            return {mul, shift};
        }
    }
    assert(false);
}

int main(int argc, char **argv)
{
    assert(argc == 3);
    int x = atoi(argv[1]);
    int y = atoi(argv[2]);
    pair<ull, ull> magic = get_magic(x, y);
    printf("%llu %llu\n", magic.first, magic.second);
}

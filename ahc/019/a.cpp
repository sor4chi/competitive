#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
}
template <class T, class... Ts>
void println(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (int i = 0; i < n; i++)
#define rep1(i, n) for (int i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

int D;
vector<vector<int>> f1, r1, f2, r2;

struct Pos {
    int x, y, z;
    bool operator<(const Pos& p) const {
        if (x != p.x) return x < p.x;
        if (y != p.y) return y < p.y;
        return z < p.z;
    }
    bool operator==(const Pos& p) const { return x == p.x && y == p.y && z == p.z; }
    Pos operator+(const Pos& p) const { return {x + p.x, y + p.y, z + p.z}; }
    Pos operator-(const Pos& p) const { return {x - p.x, y - p.y, z - p.z}; }
};

void print_3d_vector(const vector<vector<vector<int>>>& b) {
    rep(i, D) {
        rep(j, D) {
            rep(k, D) {
                cout << b[i][j][k];
                if (i != D - 1 || j != D - 1 || k != D - 1) cout << ' ';
            }
        }
    }
    cout << endl;
}

void init() {
    input(D);
    f1.resize(D, vector<int>(D));
    r1.resize(D, vector<int>(D));
    f2.resize(D, vector<int>(D));
    r2.resize(D, vector<int>(D));
    rep(i, D) {
        string s;
        input(s);
        rep(j, D) f1[i][j] = s[j] - '0';
    }
    rep(i, D) {
        string s;
        input(s);
        rep(j, D) r1[i][j] = s[j] - '0';
    }
    rep(i, D) {
        string s;
        input(s);
        rep(j, D) f2[i][j] = s[j] - '0';
    }
    rep(i, D) {
        string s;
        input(s);
        rep(j, D) r2[i][j] = s[j] - '0';
    }
}

enum Direction {
    X_PLUS,
    X_MINUS,
    Y_PLUS,
    Y_MINUS,
    Z_PLUS,
    Z_MINUS,
};

enum Rotation {
    ROLL,
    PITCH,
    YAW,
};

map<Direction, Pos> dirs = {
    {X_PLUS, {1, 0, 0}},
    {X_MINUS, {-1, 0, 0}},
    {Y_PLUS, {0, 1, 0}},
    {Y_MINUS, {0, -1, 0}},
    {Z_PLUS, {0, 0, 1}},
    {Z_MINUS, {0, 0, -1}},
};

map<Direction, map<Rotation, Direction>> relative_dirs = {
    {X_PLUS, {{ROLL, X_PLUS}, {PITCH, Y_PLUS}, {YAW, Z_PLUS}}},
    {X_MINUS, {{ROLL, X_MINUS}, {PITCH, Y_MINUS}, {YAW, Z_MINUS}}},
    {Y_PLUS, {{ROLL, Y_PLUS}, {PITCH, X_MINUS}, {YAW, Z_PLUS}}},
    {Y_MINUS, {{ROLL, Y_MINUS}, {PITCH, X_PLUS}, {YAW, Z_MINUS}}},
    {Z_PLUS, {{ROLL, Z_PLUS}, {PITCH, X_PLUS}, {YAW, Y_MINUS}}},
    {Z_MINUS, {{ROLL, Z_MINUS}, {PITCH, X_MINUS}, {YAW, Y_PLUS}}},
};

class IdGenerator {
   public:
    IdGenerator() : id(0) {}
    int generate() { return id++; }

   private:
    int id;
};

struct Block {
    int id;
    // ブロックの座標
    vector<Pos> poses_1, poses_2;
    // ブロックの座標の集合
    set<Pos> poses_1_set, poses_2_set;
    // ブロックの向き
    Direction dir_1, dir_2;
};

random_device seed_gen;
mt19937 engine(seed_gen());
unsigned long rng() {
    return engine();
}

Pos get_random_pos() {
    return {rng() % D, rng() % D, rng() % D};
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    init();

    IdGenerator id_gen;
    Block block;
    block.id = id_gen.generate();
    // 1と2からランダムな一点を選ぶ
    Pos first_pos_1 = get_random_pos();
    while (f1[first_pos_1.z][first_pos_1.x] == 0 || r1[first_pos_1.z][first_pos_1.y] == 0) {
        first_pos_1 = get_random_pos();
    }
    Pos first_pos_2 = get_random_pos();
    while (f2[first_pos_2.z][first_pos_2.x] == 0 || r2[first_pos_2.z][first_pos_2.y] == 0) {
        first_pos_2 = get_random_pos();
    }
    block.poses_1.push_back(first_pos_1);
    block.poses_2.push_back(first_pos_2);
    block.poses_1_set.insert(first_pos_1);
    block.poses_2_set.insert(first_pos_2);
    // 6方向に広げる
    // 行ける方向を探す
    vector<Direction> can_go_1, can_go_2;
    for (auto [dir, d] : dirs) {
        Pos next_pos_1 = first_pos_1 + d;
        if (next_pos_1.x < 0 || next_pos_1.x >= D || next_pos_1.y < 0 || next_pos_1.y >= D || next_pos_1.z < 0 || next_pos_1.z >= D) continue;
        if (block.poses_1_set.count(next_pos_1) == 0 && f1[next_pos_1.z][next_pos_1.x] == 1 && r1[next_pos_1.z][next_pos_1.y] == 1) {
            can_go_1.push_back(dir);
        }
        Pos next_pos_2 = first_pos_2 + d;
        if (next_pos_2.x < 0 || next_pos_2.x >= D || next_pos_2.y < 0 || next_pos_2.y >= D || next_pos_2.z < 0 || next_pos_2.z >= D) continue;
        if (block.poses_2_set.count(next_pos_2) == 0 && f2[next_pos_2.z][next_pos_2.x] == 1 && r2[next_pos_2.z][next_pos_2.y] == 1) {
            can_go_2.push_back(dir);
        }
    }
    if (can_go_1.size() == 0 || can_go_2.size() == 0) {
        exit(1);
    }
    // 進む方向をランダムに選ぶ
    uniform_int_distribution<int> dist_1(0, can_go_1.size() - 1);
    block.dir_1 = can_go_1[dist_1(engine)];
    uniform_int_distribution<int> dist_2(0, can_go_2.size() - 1);
    block.dir_2 = can_go_2[dist_2(engine)];
    // 進む
    block.poses_1.push_back(first_pos_1 + dirs[block.dir_1]);
    block.poses_2.push_back(first_pos_2 + dirs[block.dir_2]);
    block.poses_1_set.insert(first_pos_1 + dirs[block.dir_1]);
    block.poses_2_set.insert(first_pos_2 + dirs[block.dir_2]);

    while (true) {
        // 行ける方向を探す
        vector<pair<Direction, Direction>> can_go;
        for (auto [dir, d] : dirs) {
            // ブロックの方向からの相対方向を求める
            Direction next_dir_1 = relative_dirs[block.dir_1][dir];
            Pos next_pos_1 = block.poses_1.back() + d;
            if (next_pos_1.x < 0 || next_pos_1.x >= D || next_pos_1.y < 0 || next_pos_1.y >= D || next_pos_1.z < 0 || next_pos_1.z >= D) continue;
            Direction next_dir_2 = relative_dirs[block.dir_2][dir];
            Pos next_pos_2 = block.poses_2.back() + d;
            if (next_pos_2.x < 0 || next_pos_2.x >= D || next_pos_2.y < 0 || next_pos_2.y >= D || next_pos_2.z < 0 || next_pos_2.z >= D) continue;
            if (block.poses_1_set.count(next_pos_1) == 0 && f1[next_pos_1.z][next_pos_1.x] == 1 && r1[next_pos_1.z][next_pos_1.y] == 1 && block.poses_2_set.count(next_pos_2) == 0 && f2[next_pos_2.z][next_pos_2.x] == 1 && r2[next_pos_2.z][next_pos_2.y] == 1) {
                can_go.push_back({next_dir_1, next_dir_2});
            }
        }
        if (can_go.size() == 0) {
            break;
        }
        cerr << can_go.size() << endl;
        // 進む方向をランダムに選ぶ
        uniform_int_distribution<int> dist(0, can_go.size() - 1);
        auto [next_dir_1, next_dir_2] = can_go[dist(engine)];
        // 進む
        block.poses_1.push_back(block.poses_1.back() + dirs[next_dir_1]);
        block.poses_2.push_back(block.poses_2.back() + dirs[next_dir_2]);
        block.poses_1_set.insert(block.poses_1.back());
        block.poses_2_set.insert(block.poses_2.back());
    }

    vector<vector<vector<int>>> ans1(D, vector<vector<int>>(D, vector<int>(D, 0))), ans2(D, vector<vector<int>>(D, vector<int>(D, 0)));
    for (auto pos : block.poses_1) {
        ans1[pos.x][pos.y][pos.z] = 1;
    }
    for (auto pos : block.poses_2) {
        ans2[pos.x][pos.y][pos.z] = 1;
    }
    println(block.poses_1.size());
    print_3d_vector(ans1);
    print_3d_vector(ans2);

    return 0;
}

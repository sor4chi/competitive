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
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

const map<char, pair<int, int>> ds = {
    {'L', {0, -1}},
    {'R', {0, 1}},
    {'U', {-1, 0}},
    {'D', {1, 0}},
};

const int SIZE = 50;

struct Node {
    pair<int, int> p;
    vector<pair<int, int>> path;
    set<int> visited_tiles;
    int total_point;
};

string path_to_lrud(const vector<pair<int, int>>& path) {
    string res;
    pair<int, int> prev = {-1, -1};
    for (auto [i, j] : path) {
        if (prev.first != -1) {
            int di = i - prev.first;
            int dj = j - prev.second;
            if (di == 1) {
                res.push_back('D');
            } else if (di == -1) {
                res.push_back('U');
            } else if (dj == 1) {
                res.push_back('R');
            } else if (dj == -1) {
                res.push_back('L');
            }
        }
        prev = {i, j};
    }
    return res;
}

void answer(const vector<pair<int, int>>& path) {
    println(path_to_lrud(path));
}

struct Solver {
    int si, sj;
    vector<vector<int>> tiles;
    vector<vector<int>> points;
    Node best;
    Solver(int si, int sj, vector<vector<int>> tiles, vector<vector<int>> points)
        : si(si), sj(sj), tiles(tiles), points(points) {
    }

    void solve() {
        stack<Node> s;
        Node start = {{si, sj}, {{si, sj}}, {}, points[si][sj]};
        s.push(start);
        best = start;
        chrono::system_clock::time_point start_time = chrono::system_clock::now();
        while (!s.empty() && chrono::system_clock::now() - start_time < 1980ms) {
            auto [p, path, visited_tiles, total_point] = s.top();
            // {
            //     // report the best path
            //     pair<int, int> prev = {-1, -1};
            //     print("path: ");
            //     for (auto [i, j] : path) {
            //         print(i, j, ' ');
            //     }
            //     println();
            //     print("path_to_lrud: ");
            //     println(path_to_lrud(path));
            //     // report the best point
            //     print("total_point: ");
            //     println(total_point);
            //     // report visited tiles
            //     print("visited_tiles: ");
            //     for (auto tile : visited_tiles) {
            //         print(tile, ' ');
            //     }
            //     println();
            // }
            s.pop();
            int tile = tiles[p.first][p.second];
            visited_tiles.insert(tile);
            if (total_point > best.total_point) {
                best = {p, path, visited_tiles, total_point};
            }
            for (auto [_d, dij] : ds) {
                int ni = p.first + dij.first;
                int nj = p.second + dij.second;
                if (ni < 0 || ni >= SIZE || nj < 0 || nj >= SIZE) {
                    continue;
                }
                if (visited_tiles.count(tiles[ni][nj])) {
                    continue;
                }
                vector<pair<int, int>> new_path = path;
                new_path.push_back({ni, nj});
                set<int> new_visited_tiles = visited_tiles;
                new_visited_tiles.insert(tiles[ni][nj]);
                s.push({{ni, nj}, new_path, visited_tiles, total_point + points[ni][nj]});
            }
        }
    }
};

int main() {
    int si, sj;
    input(si, sj);
    vector<vector<int>> tiles(SIZE, vector<int>(SIZE));
    rep(i, SIZE) rep(j, SIZE) input(tiles[i][j]);
    vector<vector<int>> points(SIZE, vector<int>(SIZE));
    rep(i, SIZE) rep(j, SIZE) input(points[i][j]);

    Solver s = Solver(si, sj, tiles, points);
    s.solve();
    answer(s.best.path);

    return 0;
}

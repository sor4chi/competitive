#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
template <class T, class... Ts>
void println(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
}
void println() { cout << '\n'; }
template <class T, class... Ts>
void eprintln(const T& a, const Ts&... b) {
    cerr << a;
    (void)(cerr << ... << (cerr << ' ', b));
    cerr << '\n';
}
template <class T>
void eprintv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cerr << x << sep);
    }
    cerr << end;
}
void eprintln() { cerr << '\n'; }
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
#define rep(i, n) for (int i = 0; i < n; i++)
#define rep1(i, n) for (int i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

struct Edge {
    int u, v, weight;
};

vector<int> bfs(int start, const unordered_map<int, vector<int>>& path) {
    vector<int> dist(path.size(), -1);
    queue<int> q;
    q.push(start);
    dist[start] = 0;

    while (!q.empty()) {
        int u = q.front();
        q.pop();

        for (int v : path.at(u)) {
            if (dist[v] == -1) {
                dist[v] = dist[u] + 1;
                q.push(v);
            }
        }
    }

    return dist;
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    int n;
    input(n);

    // 隣接リストを作成
    unordered_map<int, vector<int>> path(n);
    rep(i, n - 1) {
        int a, b;
        input(a, b);
        a--;
        b--;
        path[a].push_back(b);
        path[b].push_back(a);
    }

    int start_node = 0;
    // 任意のノードから最も遠いノードを見つける（木の直径の一端）
    vector<int> dist_from_start = bfs(start_node, path);
    int farthest_node = max_element(dist_from_start.begin(), dist_from_start.end()) - dist_from_start.begin();

    // そのノードから最も遠いノードまでの距離を計算する（木の直径）
    vector<int> dist_from_farthest = bfs(farthest_node, path);
    int other_farthest_node = max_element(dist_from_farthest.begin(), dist_from_farthest.end()) - dist_from_farthest.begin();

    // 各ノードから木の直径の一端までの距離を計算する
    vector<int> dist_from_other = bfs(other_farthest_node, path);

    // 全てのペアの距離を計算するのを効率化
    priority_queue<Edge, vector<Edge>, function<bool(Edge, Edge)>> pq([](Edge a, Edge b) { return a.weight < b.weight; });
    rep(u, n) {
        rep(v, n) {
            if (u < v) {
                int distance = min(dist_from_farthest[u] + dist_from_farthest[v], dist_from_other[u] + dist_from_other[v]);
                pq.push({u, v, distance});
            }
        }
    }

    // マッチングをする
    vector<bool> used(n, false);
    vector<pair<int, int>> result;
    while (!pq.empty() && result.size() < n / 2) {
        auto edge = pq.top();
        pq.pop();
        int u = edge.u;
        int v = edge.v;
        if (!used[u] && !used[v]) {
            used[u] = true;
            used[v] = true;
            result.push_back({u, v});
        }
    }

    for (auto& p : result) {
        println(p.first + 1, p.second + 1);
    }

    return 0;
}

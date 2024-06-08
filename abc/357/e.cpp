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
template <class T>
void printv(const T& a, string sep = " ", string end = "\n") {
    for (auto x : a) {
        (void)(cout << x << sep);
    }
    cout << end;
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void println() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);
    vector<ll> a(n);
    // 有向グラフ
    map<ll, ll> g;
    map<ll, vector<ll>> g_rev;
    rep(i, n) {
        input(a[i]);
        a[i]--;
        g[i] = a[i];
        g_rev[a[i]].push_back(i);
    }

    map<ll, ll> cache;
    // 有向グラフからサイクル検出をし、検出したサイクルをgから削除する
    set<ll> visited;
    rep(i, n) {
        if (visited.find(i) != visited.end()) {
            continue;
        }
        vector<ll> path;
        ll cur = i;
        while (visited.find(cur) == visited.end()) {
            visited.insert(cur);
            path.push_back(cur);
            cur = g[cur];
        }

        ll cycle_start = *visited.find(cur);
        // visited(cur)以降がサイクルなので抽出
        vector<ll> cycle;
        bool is_cycle = false;
        rep(j, path.size()) {
            if (path[j] == cycle_start) {
                is_cycle = true;
            }
            if (is_cycle) {
                cycle.push_back(path[j]);
            }
        }

        // printv(cycle);

        for (auto c : cycle) {
            cache[c] = cycle.size();
            g.erase(c);
        }
    }

    // 残ったgを辿る
    for (auto [k, v] : g) {
        if (cache.find(k) != cache.end()) {
            continue;
        }
        ll prev = k;
        ll cur = v;
        // g[cur]がなくなるまで辿る
        while (g.find(cur) != g.end()) {
            prev = cur;
            // cerr << "cur: " << cur << endl;
            cur = g[cur];
        }
        ll dist = cache[cur] + 1;
        // cerr << "dist: " << dist << endl;
        // 幅優先探索
        queue<pair<ll, ll>> q;
        q.push({prev, dist});
        while (!q.empty()) {
            auto [cur, dist] = q.front();
            q.pop();
            cache[cur] = dist;
            // cerr << "cur: " << cur << " dist: " << dist << endl;
            for (auto next : g_rev[cur]) {
                if (cache.find(next) == cache.end()) {
                    q.push({next, dist + 1});
                }
            }
        }
    }

    // // print cache
    // for (auto [k, v] : cache) {
    //     print(k, v);
    //     println();
    // }

    // print sum cache value
    ll sum = 0;
    for (auto [k, v] : cache) {
        sum += v;
    }
    println(sum);

    return 0;
}

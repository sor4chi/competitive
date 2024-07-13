#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
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
#define rep(i, n) for (ll i = 0; i < n; i++)
#define rep1(i, n) for (ll i = 1; i <= n; i++)
#define yesno(a) cout << (a ? "Yes" : "No") << '\n';
#define YESNO(a) cout << (a ? "YES" : "NO") << '\n';

struct Edge {
    ll u, v, weight;
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, m;
    input(n, m);
    vector<ll> a(n);
    rep(i, n) input(a[i]);  // ノードの重み
    vector<Edge> all_edge(m);
    rep(i, m) {
        input(all_edge[i].u, all_edge[i].v, all_edge[i].weight);
        all_edge[i].u--;
        all_edge[i].v--;
    }
    map<ll, vector<pair<ll, ll>>> edges;
    for (auto e : all_edge) {
        edges[e.u].push_back({e.v, e.weight});
        edges[e.v].push_back({e.u, e.weight});
    }

    // ダイクストラで0~各ノードまでの最短距離を求める
    deque<ll> mincost(n, LLONG_MAX);
    mincost[0] = a[0];
    priority_queue<pair<ll, ll>, vector<pair<ll, ll>>, greater<pair<ll, ll>>> que;
    que.push({0, 0});
    while (!que.empty()) {
        auto [cost, node] = que.top();
        que.pop();
        if (mincost[node] < cost) continue;
        for (auto [next, weight] : edges[node]) {
            if (mincost[next] > mincost[node] + weight + a[next]) {
                mincost[next] = mincost[node] + weight + a[next];
                que.push({mincost[next], next});
            }
        }
    }

    mincost.pop_front();
    printv(mincost);

    return 0;
}

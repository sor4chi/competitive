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

double dist(ll x1, ll y1, ll x2, ll y2) {
    return sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2));
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, m, k;
    input(n, m, k);
    vector<ll> x(n), y(n);
    rep(i, n) input(x[i], y[i]);  // 電波塔の座標
    vector<ll> u(m), v(m), w(m);
    map<pair<ll, ll>, ll> uv_rev;
    rep(i, m) {
        ll ui, vi, wi;
        input(ui, vi, wi);
        ui--;
        vi--;
        if (ui > vi) swap(ui, vi);
        u[i] = ui;
        v[i] = vi;
        w[i] = wi;
        // 通信ケーブルの座標 u - v の間に重み w で通信ケーブルがある
    }
    rep(i, m) uv_rev[{u[i], v[i]}] = i;
    vector<ll> a(k), b(k);
    rep(i, k) input(a[i], b[i]);  // 住民の座標

    vector<ll> best_powers(n, 5000);
    vector<bool> used(m, 0);

    // 住民のidxと、それをカバーしている電波塔のidxを記録
    map<ll, set<ll>> mp;
    // 逆に、電波塔のidxと、それをカバーしている住民のidxを記録
    map<ll, set<ll>> mp2;
    rep(i, k) {
        // もし住民iと電波塔jの距離がpower[j]以下ならば、mp[i]にjを追加
        rep(j, n) {
            if (dist(x[j], y[j], a[i], b[i]) <= 5000) {
                mp[i].insert(j);
                mp2[j].insert(i);
            }
        }
    }

    chrono::system_clock::time_point start_time = chrono::system_clock::now();
    int iter = 0;
    vector<ll> left_towers;
    rep(i, n) left_towers.push_back(i);
    while (chrono::duration_cast<chrono::milliseconds>(chrono::system_clock::now() - start_time).count() < 1900) {
        iter++;
        // 電波塔を一つ選ぶ
        ll idx = left_towers[rand() % left_towers.size()];

        // 電波塔のpowerを試しに減らす
        ll idx_power = best_powers[idx] - 100;
        set<ll> idx_managed_residents = mp2[idx];
        // 電波塔のpowerを減らしてもだいじょうぶかどうか
        bool ok = 1;
        vector<ll> goodbye_residents;
        for (auto i : idx_managed_residents) {
            // もし住民iが電波塔idxにカバーされているならば、その距離がpower[idx]以下かどうか
            if (dist(x[idx], y[idx], a[i], b[i]) <= idx_power) continue;
            goodbye_residents.push_back(i);
            // もしカバーされていなくても、他の電波塔でカバーされているかどうか
            if (mp[i].size() > 1) continue;
            ok = 0;
            break;
        }
        if (!ok) continue;
        // 電波塔のpowerを減らす
        best_powers[idx] = idx_power;
        // 電波塔のpowerを減らすことでカバーされなくなった住民を削除
        for (auto i : goodbye_residents) {
            mp[i].erase(idx);
            mp2[idx].erase(i);
        }
        // もし電波塔idxのpowerが0になったら、left_towersから削除
        if (idx_power < 0) {
            best_powers[idx] = 0;
            left_towers.erase(find(left_towers.begin(), left_towers.end(), idx));
            continue;
        }
    }
    cerr << "iter: " << iter << endl;

    rep(i, m) used[i] = 0;

    // 最短重み総和で通信ケーブルを使って、power[i]>0の電波塔を繋げるようにグラフを作る
    // 探索しやすいように隣接mapを作る
    map<ll, set<ll>> adj;
    rep(i, m) {
        adj[u[i]].insert(v[i]);
        adj[v[i]].insert(u[i]);
    }

    // edgesを作る、コストはw[i]
    vector<tuple<ll, ll, ll>> edges;
    rep(i, m) {
        edges.push_back({w[i], u[i], v[i]});
    }

    // コストが小さい順にソート
    sort(edges.begin(), edges.end());

    // UnionFindを使って、全ての電波塔が繋がるようにする
    map<ll, set<ll>> tree;
    dsu uf(n);
    for (auto [cost, u, v] : edges) {
        if (uf.same(u, v)) continue;
        uf.merge(u, v);
        if (u > v) swap(u, v);
        // used_edgesを使って木を作る
        tree[u].insert(v);
        tree[v].insert(u);
    }

    set<ll> needs;
    rep(i, n) if (best_powers[i] > 0) needs.insert(i);

    // 木をbfsする
    queue<vector<ll>> q;
    vector<bool> visited(n, 0);
    ll start = 0;
    visited[start] = 1;
    q.push({start});
    while (q.size()) {
        auto cur = q.front();
        q.pop();
        ll u = cur.back();
        if (needs.find(u) != needs.end()) {
            rep(i, cur.size() - 1) {
                ll prev_idx = cur[i];
                ll idx = cur[i + 1];
                if (prev_idx > idx) swap(prev_idx, idx);
                ll edge_idx = uv_rev[{prev_idx, idx}];
                used[edge_idx] = 1;
            }
            needs.erase(u);
        }
        for (auto v : tree[u]) {
            if (visited[v]) continue;
            visited[v] = 1;
            vector<ll> next = cur;
            next.push_back(v);
            q.push(next);
        }
    }

    printv(best_powers);
    printv(used);

    return 0;
}

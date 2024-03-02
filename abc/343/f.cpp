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

// セグメント木を作る
// 区間lrで2番目に大きい値を出力する
using Node = struct {
    // 最大値
    ll max;
    // 最大値の個数
    ll max_count;
    // 2番目に大きい値
    ll second_max;
    // 2番目に大きい値の個数
    ll second_max_count;
};

// 2つのノードをマージする
Node op(Node a, Node b) {
    map<ll, ll> m;  // 数値とその個数
    m[a.max] += a.max_count;
    m[a.second_max] += a.second_max_count;
    m[b.max] += b.max_count;
    m[b.second_max] += b.second_max_count;
    vector<pair<ll, ll>> v(m.begin(), m.end());
    sort(v.begin(), v.end(), greater<pair<ll, ll>>());
    ll len = v.size();
    if (len == 1) {
        return Node{v[0].first, v[0].second, 0, 0};
    } else {
        return Node{v[0].first, v[0].second, v[1].first, v[1].second};
    }
}

// 単位元
Node e() {
    return Node{0, 0, 0, 0};
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, q;
    input(n, q);
    vector<Node> a(n);
    rep(i, n) {
        ll x;
        input(x);
        a[i] = Node{x, 1, 0, 0};
    }
    segtree<Node, op, e> seg(a);
    rep(i, q) {
        ll t, arg1, arg2;
        input(t, arg1, arg2);
        if (t == 1) {
            seg.set(arg1 - 1, Node{arg2, 1, 0, 0});
        } else {
            println(seg.prod(arg1 - 1, arg2).second_max_count);
        }
    }

    return 0;
}

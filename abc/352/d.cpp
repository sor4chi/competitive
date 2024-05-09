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

using Node = struct {
    // 最大値
    ll max;
    // 最小値
    ll min;
};

// 2つのノードをマージする
Node op(Node a, Node b) {
    ll max_ab = max(a.max, b.max);
    ll min_ab = min(a.min, b.min);
    return Node{max_ab, min_ab};
}

// 単位元
Node e() {
    return Node{0, (ll)1e18};
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n, k;
    input(n, k);
    vector<ll> p(n);
    rep(i, n) input(p[i]);
    // セグ木
    segtree<Node, op, e> seg(n);
    rep(i, n) {
        seg.set(p[i] - 1, Node{i, i});
    }

    // pの連続部分列で長さkの完全順列を列挙する
    ll minimum = 1e18;
    rep(i, n - k + 1) {
        Node node = seg.prod(i, i + k);
        minimum = min(minimum, node.max - node.min);
    }

    println(minimum);

    return 0;
}

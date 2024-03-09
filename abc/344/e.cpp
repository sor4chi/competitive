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

// ただのセグメント木
struct Node {
    vector<ll> value;
};

Node op(Node a, Node b) {
    // merge
    vector<ll> v;
    for (auto x : a.value) {
        v.push_back(x);
    }
    for (auto x : b.value) {
        v.push_back(x);
    }
    return Node{v};
}

Node e() {
    return Node{{}};
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll n;
    input(n);
    // // セグ木
    // segtree<Node, op, e> seg(n);
    // rep(i, n) {
    //     ll a;
    //     input(a);
    //     seg.set(i, Node{{a}});
    // }
    map<ll, ll> mp, mp_rev;
    ll prev = -1;
    ll first;
    rep(i, n) {
        ll a;
        input(a);
        if (prev == -1) {
            first = a;
            prev = a;
        } else {
            mp[prev] = a;
            mp_rev[a] = prev;
            prev = a;
        }
    }
    ll q;
    input(q);
    rep(i, q) {
        ll t;
        input(t);
        if (t == 1) {
            ll l, r;
            input(l, r);
            // insert r after l
            auto next = mp[l];
            mp[l] = r;
            mp[r] = next;
            mp_rev[next] = r;
            mp_rev[r] = l;

        } else {
            ll l;
            input(l);
            // delete l
            auto prev = mp_rev[l];
            auto next = mp[l];
            if (prev == 0) {
                first = next;
            }
            mp[prev] = next;
            mp_rev[next] = prev;

            mp.erase(l);
            mp_rev.erase(l);
        }
    }

    ll cur = first;
    string s = "";
    while (true) {
        s += to_string(cur) + " ";
        cur = mp[cur];
        if (cur == 0) {
            break;
        }
    }
    if (s.size() > 0) {
        s.pop_back();
    }
    println(s);

    return 0;
}

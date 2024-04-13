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

struct Chunk {
    ll L, R;
    bool operator<(const Chunk& other) const {
        return L < other.L;
    }
};

ll pow(ll x, ll n) {
    ll res = 1;
    while (n > 0) {
        if (n & 1) {
            res *= x;
        }
        x *= x;
        n >>= 1;
    }
    return res;
}

int main() {
    ll L, R;
    input(L, R);
    set<Chunk> left_segments = {{L, R}};
    set<Chunk> segments = {};

    ll len = R - L;
    ll sqrt_len = sqrt(len);

    rep(i, sqrt_len + 1) {
        int rev_i = pow(2, sqrt_len - i);
        for (auto left_chunk : left_segments) {
            ll left = left_chunk.L;
            ll right = left_chunk.R;
            ll left_new = (left + rev_i - 1) / rev_i * rev_i;
            if (left_new + rev_i <= right) {
                segments.insert({left_new, left_new + rev_i});
                left_segments.erase({left, right});
                left_segments.insert({left, left_new});
                left_segments.insert({left_new + rev_i, right});
            }
        }
    }

    println(segments.size());
    for (auto& seg : segments) {
        println(seg.L, seg.R);
    }

    return 0;
}

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
    cin >> n;
    vector<ll> a(n);
    map<ll, ll> miss_pos;
    rep(i, n) {
        cin >> a[i];
        if (i != a[i] - 1) {
            miss_pos[a[i]] = i;
        }
    }

    vector<pair<ll, ll>> ans;
    while (!miss_pos.empty()) {
        ll pos = miss_pos.begin()->first;
        ll idx = miss_pos.begin()->second;
        miss_pos.erase(pos);
        if (idx == pos - 1) {
            continue;
        }
        if (idx + 1 > pos) {
            ans.push_back({pos, idx + 1});
        } else {
            ans.push_back({idx + 1, pos});
        }
        a[idx] = a[pos - 1];
        miss_pos[a[idx]] = idx;
    }

    println(ans.size());
    for (auto [x, y] : ans) {
        println(x, y);
    }

    return 0;
}

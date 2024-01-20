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

void print_map(map<ll, ll> m) {
    for (auto [k, v] : m) {
        print(k, ' ', v, '\n');
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    ll N;
    input(N);
    vector<ll> A(N);
    map<ll, ll> back_of, front_of;
    rep(i, N) input(A[i]);
    rep(i, N) {
        if (A[i] == -1) {
            continue;
        }
        back_of[A[i]] = i + 1;
        front_of[i + 1] = A[i];
    }

    // deque<ll> ans;
    vector<ll> ans_right, ans_left;

    ll key = 1;
    // 右向きに探索
    while (true) {
        // backof[i] を次のkeyとして持つ
        ll v = back_of.find(key) == back_of.end() ? -1 : back_of[key];
        if (v == -1) break;
        ans_right.push_back(v);
        key = v;
    }

    // 左向きに探索
    key = 1;
    while (true) {
        // frontof[i] を次のkeyとして持つ
        ll v = front_of.find(key) == front_of.end() ? -1 : front_of[key];
        if (v == -1) break;
        ans_left.push_back(v);
        key = v;
    }

    // for (auto v : ans) print(v, ' ');
    // println();
    string s;

    // print ans_right
    // for (auto v : ans_left) print(v, ' ');
    // for (ll i = ans_left.size() - 1; i >= 0; i--) print(ans_left[i], " ");
    for (ll i = ans_left.size() - 1; i >= 0; i--) s += to_string(ans_left[i]) + " ";
    // print("1 ");
    s += "1 ";
    // for (auto v : ans_right) print(v, " ");
    for (auto v : ans_right) s += to_string(v) + " ";
    println(s);

    return 0;
}

#include <bits/stdc++.h>

// #include <atcoder/all>

using namespace std;
// using namespace atcoder;
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

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    while (1) {
        ll n;
        input(n);
        if (n == 0) break;
        set<ll> required;
        string t;
        input(t);
        rep(i, t.size()) {
            if (t[i] == 'o') {
                required.insert(i);
            }
        }
        vector<ll> a(required.begin(), required.end());
        vector<ll> spans, ans;
        rep(i, a.size() - 1) {
            spans.push_back(a[i + 1] - a[i]);
        }
        sort(spans.begin(), spans.end(), greater<ll>());
        stack<ll> st;
        rep(i, spans.size()) {
            st.push(spans[i]);
        }
        ll same_count = 1;
        ll prev = -1;
        while (!st.empty()) {
            auto span = st.top();
            st.pop();
            if (prev == span) {
                same_count++;
            } else {
                same_count = 1;
            }
            if (same_count < 3) {
                ans.push_back(span);
            }
            prev = span;
        }

        printv(ans);
    }

    return 0;
}

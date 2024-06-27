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
    while (1) {
        string s1;
        cin >> s1;
        if (s1 == ".")
            return 0;
        string s2;
        cin >> s2;
        if (s1 == s2) {
            cout << "IDENTICAL" << endl;
            continue;
        }
        vector<string> ss1, ss2;
        vector<string> so1, so2;
        {
            bool inner_str = false;
            string cur = "";
            rep(i, s1.size()) {
                char c = s1[i];
                if (c != '"') {
                    cur += c;
                }
                if (c == '"' && !inner_str) {
                    so1.push_back(cur);
                    inner_str = true;
                    cur = "";
                    continue;
                }
                if (c == '"' && inner_str) {
                    ss1.push_back(cur);
                    inner_str = false;
                    cur = "";
                    continue;
                }
                if (i == s1.size() - 1) {
                    inner_str ? ss1.push_back(cur) : so1.push_back(cur);
                }
            }
        }
        {
            bool inner_str = false;
            string cur = "";
            rep(i, s2.size()) {
                char c = s2[i];
                if (c != '"') {
                    cur += c;
                }
                if (c == '"' && !inner_str) {
                    so2.push_back(cur);
                    inner_str = true;
                    cur = "";
                    continue;
                }
                if (c == '"' && inner_str) {
                    ss2.push_back(cur);
                    inner_str = false;
                    cur = "";
                    continue;
                }
                if (i == s2.size() - 1) {
                    inner_str ? ss2.push_back(cur) : so2.push_back(cur);
                }
            }
        }
        if (ss1.size() != ss2.size() || so1.size() != so2.size()) {
            cout << "DIFFERENT" << endl;
            continue;
        }
        ll wrong_cnt_s = 0;
        rep(i, ss1.size()) {
            if (ss1[i] != ss2[i]) wrong_cnt_s++;
        }
        ll wrong_cnt_o = 0;
        rep(i, so1.size()) {
            if (so1[i] != so2[i]) wrong_cnt_o++;
        }
        if (wrong_cnt_s == 1 && wrong_cnt_o == 0) {
            cout << "CLOSE" << endl;
            continue;
        } else {
            cout << "DIFFERENT" << endl;
            continue;
        }
    }
}

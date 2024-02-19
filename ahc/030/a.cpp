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
#define rep(i, n) for (int i = 0; i < n; i++)
#define rep1(i, n) for (int i = 1; i <= n; i++)

const string DEBUG_FILE = "debug.txt";
template <class T, class... Ts>
void debug(const T& a, const Ts&... b) {
    ofstream fout(DEBUG_FILE, ios::app);
    fout << a;
    (void)(fout << ... << (fout << ' ', b));
}
void debug() {
    ofstream fout(DEBUG_FILE, ios::app);
    fout << '\n';
}


struct Solver {
    int N;      // 10 <= N <= 20
    int M;      // 2 <= M <= 20
    float eps;  // 0.01 <= eps <= 0.2 (eps = 0.01k for some integer k)
    vector<vector<pair<int, int>>> mp;
    map<int, vector<pair<int, int>>> hit;
    set<pair<int, int>> seen;
    int total = 0;

    Solver() {
        cin();
        init();
    }

   private:
    void cin() {
        input(N, M, eps);
        rep(i, M) {
            int d;
            input(d);
            total += d;
            vector<pair<int, int>> v(d);
            rep(j, d) {
                int a, b;
                input(a, b);
                v.push_back({a, b});
            }
            mp.push_back(v);
        }
    }

   private:
    void init() {
    }

   private:
    int question(vector<pair<int, int>>& v) {
        string s = "";
        for (auto p : v) {
            s += to_string(p.first) + " " + to_string(p.second) + " ";
        }
        s.pop_back();
        println('q', v.size(), s);
        int res;
        input(res);
        return res;
    }

   private:
    vector<pair<int, int>> get_arounds(pair<int, int> p) {
        // 上下左右、ただし範囲外と既に見た場所は除く
        const vector<pair<int, int>> d = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};
        vector<pair<int, int>> res;
        for (auto q : d) {
            pair<int, int> r = {p.first + q.first, p.second + q.second};
            if (r.first < 0 || r.first >= N || r.second < 0 || r.second >= N) {
                continue;
            }
            if (seen.count(r) > 0) {
                continue;
            }
            res.push_back(r);
        }
        return res;
    }

   private:
    pair<int, int> get_new() {
        // 未知の場所をランダムに返す
        while (true) {
            int i = rand() % N;
            int j = rand() % N;
            if (seen.count({i, j}) == 0) {
                return {i, j};
            }
        }
    }

   public:
    void answer() {
        int hit_cnt = 0;
        string s = "";
        for (auto p : hit) {
            hit_cnt += p.second.size();
            for (auto q : p.second) {
                s += to_string(q.first) + " " + to_string(q.second) + " ";
            }
        }
        s.pop_back();
        println('a', hit_cnt, s);
    }

   public:
    void solve() {
        int cnt = 0;
        queue<pair<int, int>> q;
        pair<int, int> p = get_new();
        q.push(p);
        seen.insert(p);
        while (cnt < total) {
            if (q.empty()) {
                pair<int, int> p = get_new();
                q.push(p);
                seen.insert(p);
            }
            pair<int, int> p = q.front();
            q.pop();
            vector<pair<int, int>> v = {p};
            int res = question(v);
            if (res >= 1) {
                hit[res].push_back(p);
                cnt += res;
                for (auto r : get_arounds(p)) {
                    q.push(r);
                    seen.insert(r);
                }
            }
        }
    }
};

int main() {
    // ios::sync_with_stdio(false);
    // cin.tie(nullptr);
    // cout.tie(nullptr);
    // cout << fixed << setprecision(15);

    Solver s;
    s.solve();
    s.answer();

    return 0;
}

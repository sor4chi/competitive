#include <bits/stdc++.h>

#include <atcoder/all>

using namespace std;
using namespace atcoder;
typedef long long ll;
template <class T, class... Ts>
void print(const T& a, const Ts&... b) {
    cout << a;
    (void)(cout << ... << (cout << ' ', b));
    cout << '\n';
}
template <class... T>
void input(T&... a) { (cin >> ... >> a); }
void print() { cout << '\n'; }
#define rep(i, n) for (ll i = 0; i < n; i++)

struct Point {
    ll i, j;
};

struct Candidate {
    string keyword;
    ll match_length;
};

struct Solver {
    ll N, M;
    ll s_i, s_j;
    vector<vector<char>> A;
    vector<string> t;
    vector<Point> ans;
    map<char, vector<Point>> mp;

    Solver() {
        cin();
        init();
    }

    void cin() {
        input(N, M);
        input(s_i, s_j);
        A.resize(N, vector<char>(N));
        rep(i, N) rep(j, N) input(A[i][j]);
        t.resize(M);
        rep(i, M) input(t[i]);
    }

    void init() {
        rep(i, N) rep(j, N) {
            mp[A[i][j]].push_back({i, j});
        }
    }

    Point closest(Point cur, char c) {
        ll min_dist = 1e18;
        Point min_p;
        for (auto p : mp[c]) {
            ll dist = abs(cur.i - p.i) + abs(cur.j - p.j);
            if (dist < min_dist) {
                min_dist = dist;
                min_p = p;
            }
        }
        return min_p;
    }

    Candidate choice_keyword_from_candidates(Point cur, vector<Candidate> candidates) {
        ll min_cost = 1e18;
        Candidate min_cost_candidate;
        for (auto candidate : candidates) {
            ll total_cost = 0;
            Point cur_p = cur;
            for (auto c : candidate.keyword) {
                Point next_p = closest(cur_p, c);
                total_cost += abs(cur_p.i - next_p.i) + abs(cur_p.j - next_p.j);
                cur_p = next_p;
            }
            if (total_cost < min_cost) {
                min_cost = total_cost;
                min_cost_candidate = candidate;
            }
        }
        return min_cost_candidate;
    }

    void solve() {
        ans.push_back({s_i, s_j});
        bool is_first = true;
        ll lap = 0;
        while (t.size()) {
            string consume_keyword;
            char cur_c1, cur_c2, cur_c3, cur_c4, cur_c5;
            if (lap < 1) {
                cur_c1 = A[ans.back().i][ans.back().j];
            } else if (lap < 2) {
                cur_c1 = A[ans.back().i][ans.back().j];
                cur_c2 = A[ans[ans.size() - 2].i][ans[ans.size() - 2].j];
            } else if (lap < 3) {
                cur_c1 = A[ans.back().i][ans.back().j];
                cur_c2 = A[ans[ans.size() - 2].i][ans[ans.size() - 2].j];
                cur_c3 = A[ans[ans.size() - 3].i][ans[ans.size() - 3].j];
            } else if (lap < 4) {
                cur_c1 = A[ans.back().i][ans.back().j];
                cur_c2 = A[ans[ans.size() - 2].i][ans[ans.size() - 2].j];
                cur_c3 = A[ans[ans.size() - 3].i][ans[ans.size() - 3].j];
                cur_c4 = A[ans[ans.size() - 4].i][ans[ans.size() - 4].j];
            } else {
                cur_c1 = A[ans.back().i][ans.back().j];
                cur_c2 = A[ans[ans.size() - 2].i][ans[ans.size() - 2].j];
                cur_c3 = A[ans[ans.size() - 3].i][ans[ans.size() - 3].j];
                cur_c4 = A[ans[ans.size() - 4].i][ans[ans.size() - 4].j];
                cur_c5 = A[ans[ans.size() - 5].i][ans[ans.size() - 5].j];
            }
            // cur_cから始まるkeywordを探す
            vector<Candidate> candidates;
            for (auto keyword : t) {
                if (keyword[0] == cur_c1) {
                    candidates.push_back({keyword, 1});  // 1文字前までマッチしている
                }
                if (keyword[0] == cur_c2 && keyword[1] == cur_c1) {
                    candidates.push_back({keyword, 2});  // 2文字前までマッチしている
                }
                if (keyword[0] == cur_c3 && keyword[1] == cur_c2 && keyword[2] == cur_c1) {
                    candidates.push_back({keyword, 3});  // 3文字前までマッチしている
                }
                if (keyword[0] == cur_c4 && keyword[1] == cur_c3 && keyword[2] == cur_c2 && keyword[3] == cur_c1) {
                    candidates.push_back({keyword, 4});  // 4文字前までマッチしている
                }
                if (keyword[0] == cur_c5 && keyword[1] == cur_c4 && keyword[2] == cur_c3 && keyword[3] == cur_c2 && keyword[4] == cur_c1) {
                    candidates.push_back({keyword, 5});  // 5文字前までマッチしている
                }
            }
            // 長くマッチしたものを優先してpush_backする
            sort(candidates.begin(), candidates.end(), [](Candidate a, Candidate b) { return a.match_length > b.match_length; });

            if (candidates.size() == 0) {
                // なければ一番最初のtを消費
                consume_keyword = t[0];
            } else {
                // あれば一番短いものを消費
                Candidate candidate = choice_keyword_from_candidates(ans.back(), candidates);
                consume_keyword = candidate.keyword;
                if (!is_first) rep(i, candidate.match_length) ans.pop_back();
            }
            for (auto c : consume_keyword) {
                if (is_first) {
                    is_first = false;
                    continue;
                }
                ans.push_back(closest(ans.back(), c));
            }

            t.erase(find(t.begin(), t.end(), consume_keyword));
            lap++;
        }
    }

    void cout() {
        for (auto p : ans) {
            print(p.i, p.j);
        }
    }
};

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    Solver s;
    s.solve();
    s.cout();

    return 0;
}

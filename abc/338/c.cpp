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

void printALeftQ(vector<vector<ll>>& ALeftQ) {
    println("ALeftQ:");
    rep(i, ALeftQ.size()) {
        vector<ll> row = ALeftQ[i];
        rep(j, row.size()) {
            cout << row[j] << " ";
        }
        println();
    }
}

int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    cout << fixed << setprecision(15);

    // 冷蔵庫に N 種類の材料があります
    ll N;
    input(N);

    vector<ll> Q(N), A(N), B(N);
    rep(i, N) input(Q[i]);  // 冷蔵庫内のi番目の材料の量
    rep(i, N) input(A[i]);  // Aを作るのに必要な冷蔵庫のi番目の材料の量
    rep(i, N) input(B[i]);  // Bを作るのに必要な冷蔵庫のi番目の材料の量

    // Aをk個作った時のQの残りの量を累積して管理
    vector<vector<ll>> ALeftQ;
    // ALeftQ[0]はQ(N)をそのまま入れる
    ALeftQ.push_back(Q);
    rep1(k, 1100000) {
        // ALeftQ[k]を作るのに必要な冷蔵庫の材料の量を計算
        vector<ll> nextQ(N);
        rep(i, N) {
            if (ALeftQ[k - 1][i] < A[i]) goto A_END;
            nextQ[i] = ALeftQ[k - 1][i] - A[i];
        }
        ALeftQ.push_back(nextQ);
    }
A_END:

    // あとはAをk個作った時の残りのQで何個Bを作れるかを計算
    const ll INF = 1LL << 60;
    ll sum_max = 0;
    rep(k, ALeftQ.size()) {
        ll l_min = INF;
        rep(i, N) {
            if (ALeftQ[k][i] < B[i]) {
                l_min = 0;
                break;
            }
            if (B[i] == 0) continue;
            l_min = min(l_min, ALeftQ[k][i] / B[i]);
        }
        sum_max = max(sum_max, k + l_min);
    }

    println(sum_max);

    return 0;
}

#include <algorithm>
#include <iostream>
#include <map>
#include <queue>
#include <stack>
#include <vector>
using namespace std;

map<int, vector<int>> paths;

pair<int, int> get_deepest_from(int s) {
    // dfs
    stack<pair<int, int>> st;
    st.push({s, 0});
    vector<bool> visited(100000, false);
    int max_depth = 0;
    int max_node = 0;
    while (!st.empty()) {
        pair<int, int> p = st.top();
        st.pop();
        int node = p.first;
        int depth = p.second;
        if (visited[node]) {
            continue;
        }
        visited[node] = true;
        if (depth > max_depth) {
            max_depth = depth;
            max_node = node;
        }
        for (int next : paths[node]) {
            if (!visited[next]) {
                st.push({next, depth + 1});
            }
        }
    }
    return {max_node, max_depth};
}

int main() {
    int n;
    cin >> n;
    n--;
    for (int i = 0; i < n; i++) {
        int a, b;
        cin >> a >> b;
        paths[a].push_back(b);
        paths[b].push_back(a);
    }

    pair<int, int> deepest = get_deepest_from(1);
    int s = get_deepest_from(deepest.first).second;
    cout << s + 1 << endl;

    return 0;
}

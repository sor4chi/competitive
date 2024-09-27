#include <bits/stdc++.h>
using namespace std;

int main(int argc, char **argv) {
    if (argc < 1) {
        cerr << "Usage: " << argv[0] << " <filename>" << endl;
        return 1;
    }

    string s;
    ofstream ofs(argv[1] + string(".txt"));
    while (getline(cin, s)) {
        ofs << s << endl;
    }

    return 0;
}

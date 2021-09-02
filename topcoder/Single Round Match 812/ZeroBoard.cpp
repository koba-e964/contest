#include <cstdio>
#include <cmath>
#include <cstring>
#include <ctime>
#include <iostream>
#include <algorithm>
#include <set>
#include <vector>
#include <sstream>
#include <typeinfo>
#include <fstream>

using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"

class ZeroBoard {
public:
  vector<int> solve(int n, int m, vector<int> data) {
    int s = 0;
    REP(i, 0, n) {
      REP(j, 0, m) {
        int sgn = (i + j) % 2 == 0 ? 1 : -1;
        data[i * m + j] *= sgn;
        s += data[i * m + j];
      }
    }
    if (s != 0) {
      return vector<int>(1, -1);
    }
    vector<int> ans;
    int big = 300000;
    REP(i, 0, n) {
      REP(j, 0, m - 1) {
        int sgn = (i + j) % 2 == 0 ? 1 : -1;
        int v = data[i * m + j];
        ans.push_back(i);
        ans.push_back(j);
        ans.push_back(1);
        ans.push_back(-v * sgn + big);
        data[i * m + j] -= v;
        data[i * m + j + 1] += v;
      }
    }
    REP(i, 0, n - 1) {
      int sgn = (i + m - 1) % 2 == 0 ? 1 : -1;
      int v = data[i * m + m - 1];
      ans.push_back(i);
      ans.push_back(m - 1);
      ans.push_back(0);
      ans.push_back(-v * sgn + big);
      data[i * m + m - 1] -= v;
      data[(i + 1) * m + m - 1] += v;
    }
    REP(i, 0, n) {
      REP(j, 0, m - 1) {
        ans.push_back(i);
        ans.push_back(j);
        ans.push_back(1);
        ans.push_back(-big);
      }
    }
    REP(i, 0, n - 1) {
      ans.push_back(i);
      ans.push_back(m - 1);
      ans.push_back(0);
      ans.push_back(-big);
    }
    return ans;
  }
};

// CUT begin
ifstream data("ZeroBoard.sample");

string next_line() {
    string s;
    getline(data, s);
    return s;
}

template <typename T> void from_stream(T &t) {
    stringstream ss(next_line());
    ss >> t;
}

void from_stream(string &s) {
    s = next_line();
}

template <typename T> void from_stream(vector<T> &ts) {
    int len;
    from_stream(len);
    ts.clear();
    for (int i = 0; i < len; ++i) {
        T t;
        from_stream(t);
        ts.push_back(t);
    }
}

template <typename T>
string to_string(T t) {
    stringstream s;
    s << t;
    return s.str();
}

string to_string(string t) {
    return "\"" + t + "\"";
}

template <typename T> string to_string(vector<T> ts) {
    stringstream s;
    s << "[ ";
    for (int i = 0; i < ts.size(); ++i) {
        if (i > 0) s << ", ";
        s << to_string(ts[i]);
    }
    s << " ]";
    return s.str();
}

bool do_test(int R, int C, vector<int> data, vector<int> __expected) {
    time_t startClock = clock();
    ZeroBoard *instance = new ZeroBoard();
    vector<int> __result = instance->solve(R, C, data);
    double elapsed = (double)(clock() - startClock) / CLOCKS_PER_SEC;
    delete instance;

    if (__result == __expected) {
        cout << "PASSED!" << " (" << elapsed << " seconds)" << endl;
        return true;
    }
    else {
        cout << "FAILED!" << " (" << elapsed << " seconds)" << endl;
        cout << "           Expected: " << to_string(__expected) << endl;
        cout << "           Received: " << to_string(__result) << endl;
        return false;
    }
}

int run_test(bool mainProcess, const set<int> &case_set, const string command) {
    int cases = 0, passed = 0;
    while (true) {
        if (next_line().find("--") != 0)
            break;
        int R;
        from_stream(R);
        int C;
        from_stream(C);
        vector<int> data;
        from_stream(data);
        next_line();
        vector<int> __answer;
        from_stream(__answer);

        cases++;
        if (case_set.size() > 0 && case_set.find(cases - 1) == case_set.end())
            continue;

        cout << "  Testcase #" << cases - 1 << " ... ";
        if ( do_test(R, C, data, __answer)) {
            passed++;
        }
    }
    if (mainProcess) {
        cout << endl << "Passed : " << passed << "/" << cases << " cases" << endl;
        int T = time(NULL) - 1630545169;
        double PT = T / 60.0, TT = 75.0;
        cout << "Time   : " << T / 60 << " minutes " << T % 60 << " secs" << endl;
        cout << "Score  : " << 500 * (0.3 + (0.7 * TT * TT) / (10.0 * PT * PT + TT * TT)) << " points" << endl;
    }
    return 0;
}

int main(int argc, char *argv[]) {
    cout.setf(ios::fixed, ios::floatfield);
    cout.precision(2);
    set<int> cases;
    bool mainProcess = true;
    for (int i = 1; i < argc; ++i) {
        if ( string(argv[i]) == "-") {
            mainProcess = false;
        } else {
            cases.insert(atoi(argv[i]));
        }
    }
    if (mainProcess) {
        cout << "ZeroBoard (500 Points)" << endl << endl;
    }
    return run_test(mainProcess, cases, argv[0]);
}
// CUT end

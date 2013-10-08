#include <vector>
#include <algorithm>
#include <iostream>
using namespace std;
typedef pair<int,int> pi;//(n,m)->m n-times
vector<pi> st;
long long int size=0;
int err(const char* msg);
int main(){
	int q,l;
	cin>>q>>l;
	for(int i=0;i<q;i++){
		string ord;
		cin>>ord;
		if(ord=="Push"){
			int n,m;
			cin>>n>>m;
			st.push_back(pi(n,m));
			size+=n;
			if(size>=l)err("FULL");
		}else if(ord=="Pop"){
			int n;
			cin>>n;
			if(size<n)err("EMPTY");
			size-=n;
			while(n>0){
				if(st.size()<=0)err("EMPTY");
				pi b=st.back();
				st.pop_back();
				if(n>=b.first){
					n-=b.first;
				}else if(n<b.first){
					b.first-=n;
					n=0;
					st.push_back(b);
				}
			}
		}else if(ord=="Top"){
			if(size<=0)err("EMPTY");
			pi b=st.back();
			cout<<b.second<<endl;
		}
		else if(ord=="Size"){
			cout<<size<<endl;
		}
	}
	cout<<"SAFE"<<endl;
}

int err(const char* msg){
	cout<<msg<<endl;
	exit(0);
}

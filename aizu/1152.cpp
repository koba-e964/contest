#include <vector>
#include <utility>
#include <string>
#include <iostream>
#include <algorithm>

using namespace std;

#define  TEST 0

struct tree
{
	tree *l,*r;
	tree():l(0),r(0){}
	tree(tree* l,tree* r):l(l),r(r){}
	bool isLeaf()const{return l==(tree*)0;}
	string toString()const
	{
		if(this->isLeaf())return "x";
		return string("(")+l->toString()+" "+r->toString()+")";
	}
	bool isomorphic(const tree* another)const
	{
		if(isLeaf())
			return another->isLeaf();
		if(another->isLeaf())return false;
		return 
			(l->isomorphic(another->l)&&r->isomorphic(another->r))
			||
			(l->isomorphic(another->r)&&r->isomorphic(another->l));
	}
	void rotate()
	{
		tree* tmp=l;
		l=r;
		r=tmp;
	}
	struct IsoPred
	{
		tree*tr;
		IsoPred(tree* t):tr(t){}
		bool operator()(tree*another){return tr->isomorphic(another);}
	};
	vector<tree*> subTrees()
	{
		if(isLeaf())
		{
			return vector<tree*>(1,this);
		}
		vector<tree*> sub1=l->subTrees(),sub2=r->subTrees();
		for(int i=0,s=sub2.size();i<s;i++)
		{
			if(find_if(sub1.begin(),sub1.end(),IsoPred(sub2[i]))==sub1.end())
				sub1.push_back(sub2[i]);
		}
		if(find_if(sub1.begin(),sub1.end(),IsoPred(this))==sub1.end())
			sub1.push_back(this);
		return sub1;
	}
};

double sim(tree* tr)
{
	if(TEST)
		cout<<"sim["<<tr->toString()<<"]=";
	if(tr->isLeaf())
	{
		if(TEST)
			cout<<"0"<<endl;
		return 0.0;
	}
	vector<tree*> sub1=tr->l->subTrees();
	vector<tree*> sub2=tr->r->subTrees();
	vector<tree*> all=tr->subTrees();
	int count=0;
	for(int i=0,s=sub1.size();i<s;i++)
	{
		if(find_if(sub2.begin(),sub2.end(),tree::IsoPred(sub1[i]))!=sub2.end())
			count++;
	}
	double res=count*1.0/all.size();
	if(TEST)
		cout<<res<<endl;
	return res;
}

tree *tmax(tree*,tree*);
tree *tmin(tree*,tree*);


int compInsym(tree* a,tree *b)
{
	double sa=sim(a);
	double sb=sim(b);
	if(sa!=sb)
	{
		return sa<sb?1:-1;
	}
	if(a->isLeaf() && b->isLeaf())return 0;
	{
		tree* ma=tmax((a->l),(a->r));
		tree* mb=tmax((b->l),(b->r));
		int res=compInsym(ma,mb);
		if(res!=0)return res;
	}
	{
		tree* ma=tmin((a->l),(a->r));
		tree* mb=tmin((b->l),(b->r));
		int res=compInsym(ma,mb);
		if(res!=0)return res;
	}
	return 0;
	
}
tree *tmax(tree *a,tree *b)
{
	int res=compInsym(a,b);
	if(res>0)return a;
	return b;
}
tree *tmin(tree *a,tree *b)
{
	int res=compInsym(a,b);
	if(res<0)return a;
	return b;
}

void adjust(tree *a,bool left)
{
	if(a->isLeaf())return;
	tree *l=a->l,*r=a->r;
	int c=compInsym(l,r);
	if(TEST|1)
		cout<<"@@@"<<l->toString()<<(c==0?"==":c>0?">":"<")<<r->toString()<<endl;
	if((c<0) == left)a->rotate();
	adjust(a->l,true);
	adjust(a->r,false);
}



vector<tree*> pool;
tree *add(tree* ptr)
{
	pool.push_back(ptr);
	return ptr;
}
void clear()
{
	for(int i=0,s=pool.size();i<s;i++)
	{
		delete pool[i];
	}
	pool.clear();
}




typedef pair<tree*,int> pti;
pti parse(string str)
{
	if(str[0]=='(')
	{
		pti sub1=parse(str.substr(1));
		int off1=sub1.second+2;
		pti sub2=parse(str.substr(off1));
		tree* ret=add(new tree(sub1.first,sub2.first));
		return pti(ret,off1+sub2.second+1);
	}
	return pti(add(new tree()),1);
}




int main(void)
{
	string str;
	while(1)
	{
		getline(cin,str);
		if(str=="0")break;
		pair<tree*,int> tr=parse(str);
		adjust(tr.first,true);
		cout<<tr.first->toString()<<endl;
	}
	return 0;
}



